use printpdf::{PdfDocumentReference, Pt, Mm, PdfLayerReference, TextRenderingMode, IndirectFontRef};
use printpdf::indices::{PdfLayerIndex, PdfPageIndex};
use crate::pdf_printer::{
    pdf_utils, 
    color_manager::{
        self,
        ColorError,
    }, 
    FontLibrary,
    transforms::{self, Transform}
};
use crate::idml_parser::{
    IDMLPackage, IDMLResources,
    spread_parser::*,
    // styles_parser,
    styles::{
        paragraph_style::ParagraphStyle,
        character_style::CharacterStyle,
    },
    story_parser::{Story, 
        ParagraphStyleRange, 
        CharacterStyleRange, 
        // Content, 
        StoryContent
    },
};

#[derive(Debug,Default,Clone)]
pub struct TextRenderSettings {
    font_name: Option<String>,
    font_style: Option<String>,
    font_size: Option<f64>,
    font: Option<IndirectFontRef>,
    fill_color: Option<printpdf::Color>,
    stroke_color: Option<printpdf::Color>,
}

impl TextRenderSettings {
    fn apply_to_layer(&mut self, layer: &PdfLayerReference, font_library: &FontLibrary) { 

        // If name, style and size are some, then set font
        match (&self.font_name, &self.font_style, self.font_size) {
            (Some(name), Some(style), Some(size)) => {
                if let Some(font) = font_library.font_from_idml_name_and_style(name, style) {
                    layer.set_font(font, size);
                    self.font = Some(font.clone());
                }
            },
            _ => {}
        }        

        // Fill color
        if let Some(color) = &self.fill_color {
            layer.set_fill_color(color.clone());
        }

        // Stroke color
        if let Some(color) = &self.stroke_color {
            layer.set_outline_color(color.clone());
        }

        match (&self.stroke_color, &self.fill_color) {
            (Some(_), Some(_))  => layer.set_text_rendering_mode(TextRenderingMode::FillStroke),
            (Some(_), None)     => layer.set_text_rendering_mode(TextRenderingMode::Stroke),
            (None,    Some(_))  => layer.set_text_rendering_mode(TextRenderingMode::Fill),
            (None,    None)     => layer.set_text_rendering_mode(TextRenderingMode::Invisible) // <- This might not be the right choice
        } 
    }

    fn write_text(&mut self, layer: &PdfLayerReference, font_library: &FontLibrary, text: &str) {
        &self.apply_to_layer(layer, font_library);

        layer.set_line_height(10.0);
        
        // If name, style and size are some, then we can write the given text
        if let Some(font) = &self.font {
            layer.write_text(text, font);
        }
    }
}

impl TextFrame {
    pub fn render_story(&self,
        parent_transform: &Transform,
        pdf_doc: &PdfDocumentReference, 
        idml_package: &IDMLPackage, 
        font_library: &FontLibrary,
        page_index: &Option<PdfPageIndex>, 
        layer_index: &Option<PdfLayerIndex>
    ) -> Result<(), String> {
        
        // Get the current layer of the PDF we are working on
        let layer = pdf_utils::layer_from_index(pdf_doc, page_index, layer_index)?;

        // Keeps track of changes to things like fonts, colors, stroke width etc.
        let mut settings = TextRenderSettings::default();

        if let Some(story_id) = &self.parent_story() {
            if let Some(story) = idml_package.story_from_id(story_id) {
                // Create a text section in the pdf for the textframe
                layer.begin_text_section();
                {
                    let (x_min, y_min) = &self.topleft_point(parent_transform);
                    layer.set_text_cursor(*x_min, *y_min);

                    &self.render_paragraph_styles(story, &layer, idml_package.resources(), font_library, settings.clone(), parent_transform, pdf_doc)?;
                }
                layer.end_text_section();
            }
        }

        settings.apply_to_layer(&layer, font_library);
        
        Ok(())
    }

    fn topleft_point(&self, parent_transform: &Transform) -> (Mm, Mm) {
        let item_transform = transforms::from_vec(&self.item_transform());

        let points: Vec<(Mm, Mm)> = self.properties().into_iter()
            .filter_map(|point| point.path_geometry().as_ref())
            .map(|path_geom| path_geom.geometry_path_type().path_point_arrays())
            .flat_map(|path_point_arrays| 
                path_point_arrays.into_iter()
                .flat_map(|path_point_array| 
                    path_point_array.path_point_array().into_iter()
                    .filter_map(|path_point_type| path_point_type.anchor().as_ref())
                    .map(|point| item_transform.combine_with(&parent_transform).apply_to_point(&point[0],&point[1]))
                    .map(|point| (
                        Mm::from(Pt(point[0])), 
                        Mm::from(Pt(point[1]))) 
                    )
                ).into_iter()
            ).collect();
    
        // Get leftmost x 
        let &(x, _) = points.iter().min_by(|(x1, _), (x2, _)| 
            x1.partial_cmp(&x2).unwrap()
        ).unwrap();
        
        // Get uppermost y
        let &(_, y) = points.iter().max_by(|(_, y1), (_, y2)| 
            y1.partial_cmp(&y2).unwrap()
        ).unwrap();

        (x, y)
    } 
    
    pub fn render_paragraph_styles(&self, 
        story: &Story, 
        layer: &PdfLayerReference, 
        idml_resources: &IDMLResources,
        font_library: &FontLibrary,
        parent_settings: TextRenderSettings,
        parent_transform: &Transform,
        pdf_doc: &PdfDocumentReference,
    ) -> Result<(),String> {
        let mut settings = parent_settings;

        match story.paragraph_style_ranges() {
            Some(p_styles) => {
                for p_style in p_styles { 
                    if let Some(style_id) = p_style.applied_paragraph_style() {
                        if let Some(style) = idml_resources.styles().paragraph_style_from_id(style_id) {
                            // Apply paragraph style formats 
                            &self.apply_paragraph_style(&style, layer, idml_resources, font_library, &mut settings, parent_transform, pdf_doc)?;
                        }
                    }

                    // TODO: Apply local paragraph formats
                    
                    &self.render_character_styles(p_style, layer, idml_resources, font_library, settings.clone(), parent_transform, pdf_doc)?;
                }
            },
            None => {}
        }
        
        Ok(())
    }

    fn apply_paragraph_style(&self, 
        p_style: &ParagraphStyle,
        layer: &PdfLayerReference, 
        idml_resources: &IDMLResources,
        font_library: &FontLibrary,
        settings: &mut TextRenderSettings,
        _parent_transform: &Transform,
        _pdf_doc: &PdfDocumentReference,
    ) -> Result<(),String> {
        // Fill color
        if let Some(color_id) = p_style.fill_color() {
            let color = match color_manager::color_from_id(idml_resources, color_id){
                Ok(c) => c,
                Err(ColorError::ColorNotImplemented) => None,
                _ => {return Err("Color error".to_string())}
            };
            // layer.set_fill_color(color);
            settings.fill_color = color;
        };

        // Stroke color
        if let Some(color_id) = p_style.stroke_color() {
            let color = match color_manager::color_from_id(idml_resources, color_id) {
                Ok(c) => c,
                Err(ColorError::ColorNotImplemented) => None,
                _ => {return Err("Color error".to_string())}
            };
            // layer.set_outline_color(color);
            settings.stroke_color = color;
        };

        // Font name
        if let Some(p) = p_style.properties() {
            settings.font_name = p.applied_font().clone();
        }

        // Font style
        if p_style.font_style().is_some() {
            settings.font_style = p_style.font_style().clone();
        }

        // Font size 
        if p_style.point_size().is_some() {
            settings.font_size = p_style.point_size().clone();
        }

        Ok(())
    }
    
    pub fn render_character_styles(&self, 
        paragraph_style: &ParagraphStyleRange, 
        layer: &PdfLayerReference, 
        idml_resources: &IDMLResources,
        font_library: &FontLibrary,
        parent_settings: TextRenderSettings,
        parent_transform: &Transform,
        pdf_doc: &PdfDocumentReference,
    ) -> Result<(),String> {
        let mut settings = parent_settings;

        match paragraph_style.character_style_ranges() {
            Some(c_styles) => {
                for c_style in c_styles {
                    if let Some(style_id) = c_style.applied_character_style() {
                        if let Some(style) = idml_resources.styles().character_style_from_id(style_id) {
                            // Apply character style formats 
                            &self.apply_character_style(&style, layer, idml_resources, font_library, &mut settings, parent_transform, pdf_doc);
                        }
                    }

                    // TODO: Apply local character formats
                    
                    &self.render_story_contents(c_style, layer, idml_resources, font_library, settings.clone(), parent_transform, pdf_doc);
                } 
            },
            None => {}
        }

        Ok(())
    }

    fn apply_character_style(&self, 
        c_style: &CharacterStyle,
        layer: &PdfLayerReference, 
        idml_resources: &IDMLResources,
        font_library: &FontLibrary,
        settings: &mut TextRenderSettings,
        _parent_transform: &Transform,
        _pdf_doc: &PdfDocumentReference,
    ) -> Result<(),String> {
        // Fill color
        if let Some(color_id) = c_style.fill_color() {
            let color = match color_manager::color_from_id(idml_resources, color_id){
                Ok(c) => c,
                Err(ColorError::ColorNotImplemented) => None,
                _ => {return Err("Color error".to_string())}
            };
            // layer.set_fill_color(color);
            settings.fill_color = color;
        };

        // Stroke color
        if let Some(color_id) = c_style.stroke_color() {
            let color = match color_manager::color_from_id(idml_resources, color_id){
                Ok(c) => c,
                Err(ColorError::ColorNotImplemented) => None,
                _ => {return Err("Color error".to_string())}
            };
            // layer.set_outline_color(color);
            settings.stroke_color = color;
        };

        // Font name
        if let Some(p) = c_style.properties() {
            settings.font_name = p.applied_font().clone();
        }

        // Font style
        if c_style.font_style().is_some() {
            settings.font_style = c_style.font_style().clone();
        }

        // Font size 
        if c_style.point_size().is_some() {
            settings.font_size = c_style.point_size().clone();
        }

        Ok(())
    }

    pub fn render_story_contents(&self, 
        character_style: &CharacterStyleRange, 
        layer: &PdfLayerReference, 
        _idml_resources: &IDMLResources,
        font_library: &FontLibrary,
        parent_settings: TextRenderSettings,
        _parent_transform: &Transform,
        _pdf_doc: &PdfDocumentReference
    ) -> Result<(), String> {
        let mut settings = parent_settings.clone();

        match character_style.contents() {
            Some(contents) => {
                for content in contents {
                    match content {
                        StoryContent::Content(text) => {
                            // TODO: Actually add the correct font

                            // let font = pdf_doc.add_external_font(std::fs::File::open("/Library/Fonts/Arial Unicode.ttf").unwrap()).unwrap();
                            // layer.set_font(&font, 12.0);

                            // layer.set_line_height(10.0);
                            // layer.set_character_spacing(1.0);
                            // layer.set_text_rendering_mode(TextRenderingMode::Fill);
                            settings.write_text(layer, font_library, text);

                        },
                        StoryContent::Br => {
                            layer.add_line_break();
                        },
                        _ => {}
                    }
                }
            }
            None => {}
        }

        Ok(())
    }
}

impl IDMLPackage {
    pub fn story_from_id(&self, id: &String) -> Option<&Story> {
        
        // Search through object styles and find one matching the given id
        // Note: Maybe more effecient to implement stories as a HashMap, 
        //       to make lookups faster in the future
        let stories:Vec<&Story> = self.stories().iter().filter(|&story| story.id() == id).collect();

        match stories.len() {
            0 => {
                None
            },
            1 => {
                Some(stories[0])
            },
            _ => {
                panic!("Multiple stories match the same id '{}' ", id);
            }
        }
    }
}
