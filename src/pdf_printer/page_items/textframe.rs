use crate::idml_parser::{
    spread_parser::*,
    story_parser::*,
    styles::{
        character_style::{self, CharacterStyle},
        paragraph_style::{self, ParagraphStyle},
    },
    IDMLPackage, IDMLResources,
};
use crate::pdf_printer::pdf_utils::{self, *};
use crate::pdf_printer::{
    color_manager::{self, Color, *},
    transforms::{self, *},
};
use libharu_sys::*;
use std::ffi::CString;
use std::fmt;
use std::ptr;

#[derive(Debug, Clone)]
pub struct RenderProperties<'a> {
    idml_resources: &'a IDMLResources,
    font_name: Option<String>,
    font_style: Option<String>,
    font_size: Option<f64>,
    stroke_color: Option<Color>,
    fill_color: Option<Color>,
}

trait StyleProperties {
    fn get_applied_font(self) -> Option<String>;
}

impl StyleProperties for paragraph_style::ParagraphProperties {
    fn get_applied_font(self) -> Option<String> {
        self.applied_font().clone()
    }
}

impl StyleProperties for character_style::CharacterProperties {
    fn get_applied_font(self) -> Option<String> {
        self.applied_font().clone()
    }
}

impl<'a> RenderProperties<'a> {
    fn new(idml_resources: &'a IDMLResources) -> Self {
        RenderProperties {
            idml_resources,
            font_name: None,
            font_style: None,
            font_size: None,
            stroke_color: None,
            fill_color: None,
        }
    }

    fn set_font_name(&mut self, style_properties: Option<impl StyleProperties>) {
        if let Some(p) = style_properties {
            self.font_name = p.get_applied_font();
        }
    }

    fn set_font_style(&mut self, font_style: Option<String>) {
        if font_style.is_some() {
            self.font_style = font_style.clone();
        }
    }

    fn set_font_size(&mut self, font_size: Option<f64>) {
        if font_size.is_some() {
            self.font_size = font_size.clone();
        }
    }
    fn set_stroke_color(&mut self, stroke_color: Option<String>) -> Result<(), ColorError> {
        if let Some(color_id) = stroke_color {
            let color = match color_manager::color_from_id(self.idml_resources, &color_id) {
                Ok(c) => Some(c),
                Err(ColorError::ColorNotImplemented) => None,
                Err(e) => return Err(e),
            };
            self.stroke_color = color;
        };
        Ok(())
    }

    fn set_fill_color(&mut self, fill_color: Option<String>) -> Result<(), ColorError> {
        if let Some(color_id) = fill_color {
            let color = match color_manager::color_from_id(self.idml_resources, &color_id) {
                Ok(c) => Some(c),
                Err(ColorError::ColorNotImplemented) => None,
                Err(e) => return Err(e),
            };
            self.fill_color = color;
        };
        Ok(())
    }
}

impl TextFrame {
    pub fn render_story(
        &self,
        idml_package: &IDMLPackage,
        parent_transform: &mut Transform,
        pdf_doc: HPDF_Doc,
        current_page: HPDF_Page,
    ) -> Result<(), String> {
        if let Some(story_id) = self.parent_story() {
            if let Ok(story) = idml_package.story_from_id(story_id) {
                let render_properties = RenderProperties::new(idml_package.resources());
                unsafe {
                    HPDF_Page_BeginText(current_page);
                    &self.render_paragraph_styles(
                        story,
                        &render_properties,
                        idml_package.resources(),
                        parent_transform,
                        pdf_doc,
                        current_page,
                    )?;
                    HPDF_Page_EndText(current_page);
                }
            }
        }
        Ok(())
    }

    pub fn render_paragraph_styles(
        &self,
        story: &Story,
        parent_properties: &RenderProperties,
        idml_resources: &IDMLResources,
        parent_transform: &Transform,
        pdf_doc: HPDF_Doc,
        current_page: HPDF_Page,
    ) -> Result<(), String> {
        let mut render_properties = parent_properties.clone();

        if let Some(p_styles) = story.paragraph_style_ranges() {
            for p_style in p_styles {
                if let Some(style_id) = p_style.applied_paragraph_style() {
                    if let Some(style) = idml_resources.styles().paragraph_style_from_id(style_id) {
                        // Apply paragraph style formats
                        render_properties.set_fill_color(style.fill_color().clone());
                        render_properties.set_stroke_color(style.stroke_color().clone());
                        render_properties.set_font_name(style.properties().clone());
                        render_properties.set_font_style(style.font_style().clone());
                        render_properties.set_font_size(style.point_size().clone());
                    }
                }

                // TODO: Apply local paragraph formats

                &self.render_character_styles(
                    p_style,
                    &render_properties,
                    idml_resources,
                    parent_transform,
                    pdf_doc,
                    current_page,
                )?;
            }
        }

        Ok(())
    }

    pub fn render_character_styles(
        &self,
        paragraph_style: &ParagraphStyleRange,
        parent_properties: &RenderProperties,
        idml_resources: &IDMLResources,
        parent_transform: &Transform,
        pdf_doc: HPDF_Doc,
        current_page: HPDF_Page,
    ) -> Result<(), String> {
        let mut render_properties = parent_properties.clone();

        match paragraph_style.character_style_ranges() {
            Some(c_styles) => {
                for c_style in c_styles {
                    if let Some(style_id) = c_style.applied_character_style() {
                        if let Some(style) =
                            idml_resources.styles().character_style_from_id(style_id)
                        {
                            // Apply character style formats
                            render_properties.set_fill_color(style.fill_color().clone());
                            render_properties.set_stroke_color(style.stroke_color().clone());
                            render_properties.set_font_name(style.properties().clone());
                            render_properties.set_font_style(style.font_style().clone());
                            render_properties.set_font_size(style.point_size().clone());
                        }
                    }

                    // TODO: Apply local character formats

                    &self.render_story_contents(
                        c_style,
                        &render_properties,
                        idml_resources,
                        parent_transform,
                        pdf_doc,
                        current_page,
                    );
                }
            }
            None => {}
        }

        Ok(())
    }

    pub fn render_story_contents(
        &self,
        character_style: &CharacterStyleRange,
        parent_properties: &RenderProperties,
        idml_resources: &IDMLResources,
        parent_transform: &Transform,
        pdf_doc: HPDF_Doc,
        current_page: HPDF_Page,
    ) -> Result<(), String> {
        if let Some(contents) = character_style.contents() {
            for content in contents {
                unsafe {
                    match content {
                        StoryContent::Content(text) => {
                            // TODO: Actually add the correct font
                            // let font = pdf_doc.add_external_font(std::fs::File::open("/Library/Fonts/Arial Unicode.ttf").unwrap()).unwrap();
                            let font = HPDF_GetFont(
                                pdf_doc,
                                CString::new("Helvetica").unwrap().as_ptr(),
                                ptr::null_mut(),
                            );
                            HPDF_Page_SetFontAndSize(
                                current_page,
                                font,
                                parent_properties.font_size.unwrap_or(200_f64) as f32,
                            );
                            let (left, right, top, bottom) = boundingbox(&self, parent_transform);
                            HPDF_Page_TextRect(
                                current_page,
                                left as f32,
                                top as f32,
                                right as f32,
                                bottom as f32,
                                CString::new(text.clone()).unwrap().as_ptr(),
                                HPDF_TextAlignment::HPDF_TALIGN_LEFT,
                                ptr::null_mut(),
                            );
                        }
                        StoryContent::Br => {
                            HPDF_Page_MoveToNextLine(current_page);
                        }
                        _ => {}
                    }
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
enum StoryError {
    NoStoryMatched(String),
    MultipleStoriesMatched(String),
}

impl IDMLPackage {
    fn story_from_id(&self, id: &str) -> Result<&Story, StoryError> {
        // Search through object styles and find one matching the given id
        // Note: Maybe more effecient to implement stories as a HashMap,
        //       to make lookups faster in the future
        let stories: Vec<&Story> = self
            .stories()
            .iter()
            .filter(|&story| story.id() == id)
            .collect();

        match stories.len() {
            0 => Err(StoryError::NoStoryMatched(id.to_string())),
            1 => Ok(stories[0]),
            _ => Err(StoryError::MultipleStoriesMatched(id.to_string())),
        }
    }
}
