use printpdf::{PdfDocumentReference, Pt, Mm, TextRenderingMode, PdfLayerReference};
use printpdf::indices::{PdfLayerIndex, PdfPageIndex};
use crate::pdf_printer::transforms::{self, Transform};
use crate::idml_parser::{IDMLPackage, IDMLResources};
use crate::idml_parser::spread_parser::*;
use crate::idml_parser::story_parser::{Story, ParagraphStyleRange, CharacterStyleRange, Content, StoryContent};
use crate::pdf_printer::{pdf_utils, color_manager, FontLibrary};

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

        if let Some(story_id) = &self.parent_story() {
            if let Some(story) = idml_package.story_from_id(story_id) {
                // Create a text section in the pdf for the textframe
                layer.begin_text_section();
                {
                    let (x_min, y_min) = &self.topleft_point(parent_transform);
                    layer.set_text_cursor(*x_min, *y_min);

                    &self.apply_paragraph_style(story, &layer, idml_package.resources(), font_library, parent_transform, pdf_doc);
                }
                layer.end_text_section();
            }
        }
        
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
                    .map(|point|  (
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
    
    pub fn apply_paragraph_style(&self, 
        story: &Story, 
        layer: &PdfLayerReference, 
        idml_resources: &IDMLResources,
        font_library: &FontLibrary,
        parent_transform: &Transform,
        pdf_doc: &PdfDocumentReference,
    ) -> Result<(),String> {
        match story.paragraph_style_ranges() {
            Some(p_styles) => {
                for p_style in p_styles { 
                    // TODO: Apply paragraph style formats 
                    

                    // TODO: Apply local paragraph formats

                    &self.apply_character_style(p_style, layer, idml_resources, font_library, parent_transform, pdf_doc);
                }
            },
            None => {}
        }
        
        Ok(())
    }
    
    pub fn apply_character_style(&self, 
        paragraph_style: &ParagraphStyleRange, 
        layer: &PdfLayerReference, 
        idml_resources: &IDMLResources,
        font_library: &FontLibrary,
        parent_transform: &Transform,
        pdf_doc: &PdfDocumentReference,
    ) -> Result<(),String> {
        match paragraph_style.character_style_ranges() {
            Some(c_styles) => {
                for c_style in c_styles {
                    // TODO: Apply character style formats 

                    // TODO: Apply local character formats
                    if let Some(color) = c_style.fill_color() {
                        layer.set_fill_color(idml_resources.color_from_id(color)?);
                    }

                    &self.apply_story_content(c_style, layer, idml_resources, font_library, parent_transform, pdf_doc);
                } 
            },
            None => {}
        }

        Ok(())
    }

    pub fn apply_story_content(&self, 
        character_style: &CharacterStyleRange, 
        layer: &PdfLayerReference, 
        idml_resources: &IDMLResources,
        font_library: &FontLibrary,
        parent_transform: &Transform,
        pdf_doc: &PdfDocumentReference
    ) -> Result<(),String> {
        match character_style.contents() {
            Some(contents) => {
                for content in contents {
                    match content {
                        StoryContent::Content(text) => {
                            // TODO: Actually add the correct font

                            let font = pdf_doc.add_external_font(std::fs::File::open("/Library/Fonts/Arial Unicode.ttf").unwrap()).unwrap();
                            layer.set_font(&font, 12.0);

                            layer.set_line_height(10.0);
                            layer.set_character_spacing(1.0);
                            layer.set_text_rendering_mode(TextRenderingMode::Fill);

                            layer.write_text(text, &font);
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
