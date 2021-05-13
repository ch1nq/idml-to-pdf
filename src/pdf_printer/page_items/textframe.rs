use crate::idml_parser::{
    spread_parser::*,
    story_parser::*,
    styles::{
        character_style::{self},
        paragraph_style::{self},
    },
    IDMLPackage, IDMLResources,
};
use crate::pdf_printer::pdf_utils::*;
use crate::pdf_printer::{
    color_manager::{self, Color, *},
    font_manager::FontLibrary,
    transforms::*,
};
use libharu_sys::*;
use std::ffi::CString;
use std::ptr;

#[derive(Debug, Clone)]
pub struct RenderProperties<'a> {
    idml_resources: &'a IDMLResources,
    font_name: Option<String>,
    font_style: Option<String>,
    font_size: Option<f64>,
    auto_leading: Option<f64>,
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
            auto_leading: None,
            stroke_color: None,
            fill_color: None,
        }
    }

    fn with_font_name(&mut self, style_properties: Option<impl StyleProperties>) -> &mut Self {
        if let Some(p) = style_properties {
            let font_name = p.get_applied_font();
            if font_name.is_some() {
                self.font_name = font_name.clone();
            }
        }
        self
    }

    fn with_font_style(&mut self, font_style: Option<String>) -> &mut Self {
        if font_style.is_some() {
            self.font_style = font_style.clone();
        }
        self
    }

    fn with_font_size(&mut self, font_size: Option<f64>) -> &mut Self {
        if font_size.is_some() {
            self.font_size = font_size.clone();
        }
        self
    }

    fn with_auto_leading(&mut self, auto_leading: Option<f64>) -> &mut Self {
        if auto_leading.is_some() {
            self.auto_leading = auto_leading.clone();
        }
        self
    }

    fn with_stroke_color(&mut self, stroke_color: Option<String>) -> &mut Self {
        if let Some(color_id) = stroke_color {
            let color = match color_manager::color_from_id(self.idml_resources, &color_id) {
                Ok(c) => Some(c),
                Err(ColorError::ColorNotImplemented) => None,
                Err(_) => None,
            };
            self.stroke_color = color;
        };
        self
    }

    fn with_fill_color(&mut self, fill_color: Option<String>) -> &mut Self {
        if let Some(color_id) = fill_color {
            let color = match color_manager::color_from_id(self.idml_resources, &color_id) {
                Ok(c) => Some(c),
                Err(ColorError::ColorNotImplemented) => None,
                Err(_) => None,
            };
            self.fill_color = color;
        };
        self
    }
}

impl TextFrame {
    pub fn render_story(
        &self,
        idml_package: &IDMLPackage,
        parent_transform: &Transform,
        font_lib: &FontLibrary,
        current_page: HPDF_Page,
    ) -> Result<(), String> {
        if let Some(story_id) = self.parent_story() {
            if let Ok(story) = idml_package.story_from_id(story_id) {
                let render_properties = RenderProperties::new(idml_package.resources());
                let bb = boundingbox(&self, parent_transform);
                unsafe {
                    HPDF_Page_GSave(current_page);
                    HPDF_Page_BeginText(current_page);
                    HPDF_Page_MoveTextPos(current_page, bb.left as f32, bb.top as f32);
                    let mut has_offset = false;
                    if let Some(p_styles) = story.paragraph_style_ranges() {
                        for p_style in p_styles {
                            &self.render_paragraph_style(
                                p_style,
                                &render_properties,
                                idml_package.resources(),
                                parent_transform,
                                font_lib,
                                current_page,
                                &mut has_offset,
                            )?;
                        }
                    }
                    HPDF_Page_EndText(current_page);
                    HPDF_Page_GRestore(current_page);
                }
            }
        }
        Ok(())
    }

    pub fn render_paragraph_style(
        &self,
        p_style: &ParagraphStyleRange,
        parent_properties: &RenderProperties,
        idml_resources: &IDMLResources,
        parent_transform: &Transform,
        font_lib: &FontLibrary,
        current_page: HPDF_Page,
        has_offset: &mut bool,
    ) -> Result<(), String> {
        let mut render_properties = parent_properties.clone();

        // Apply paragraph style formats
        if let Some(style_id) = p_style.applied_paragraph_style() {
            if let Some(style) = idml_resources.styles().paragraph_style_from_id(style_id) {
                render_properties
                    .with_fill_color(style.fill_color().clone())
                    .with_stroke_color(style.stroke_color().clone())
                    .with_font_name(style.properties().clone())
                    .with_font_style(style.font_style().clone())
                    .with_font_size(style.point_size().clone())
                    .with_auto_leading(style.auto_leading().clone());
            }
        }

        // TODO: Apply local paragraph formats

        if let Some(c_styles) = p_style.character_style_ranges() {
            for c_style in c_styles {
                &self.render_character_style(
                    c_style,
                    &render_properties,
                    idml_resources,
                    parent_transform,
                    font_lib,
                    current_page,
                    has_offset,
                )?;
            }
        }

        Ok(())
    }

    pub fn render_character_style(
        &self,
        c_style: &CharacterStyleRange,
        parent_properties: &RenderProperties,
        idml_resources: &IDMLResources,
        parent_transform: &Transform,
        font_lib: &FontLibrary,
        current_page: HPDF_Page,
        has_offset: &mut bool,
    ) -> Result<(), String> {
        let mut render_properties = parent_properties.clone();

        // Apply character style formats
        if let Some(style_id) = c_style.applied_character_style() {
            if let Some(style) = idml_resources.styles().character_style_from_id(style_id) {
                render_properties
                    .with_fill_color(style.fill_color().clone())
                    .with_stroke_color(style.stroke_color().clone())
                    .with_font_name(style.properties().clone())
                    .with_font_style(style.font_style().clone())
                    .with_font_size(style.point_size().clone())
                    .with_auto_leading(style.auto_leading().clone());
            }
        }

        // Apply local character formats
        render_properties
            .with_fill_color(c_style.fill_color().clone())
            .with_stroke_color(c_style.stroke_color().clone())
            // .with_font_name(c_style.properties().clone())
            .with_font_style(c_style.font_style().clone())
            .with_font_size(c_style.point_size().clone());

        if let Some(contents) = c_style.contents() {
            for content in contents {
                &self.render_story_content(
                    content,
                    &render_properties,
                    parent_transform,
                    font_lib,
                    current_page,
                    has_offset,
                );
            }
        }
        Ok(())
    }

    pub fn render_story_content(
        &self,
        content: &StoryContent,
        render_properties: &RenderProperties,
        parent_transform: &Transform,
        font_lib: &FontLibrary,
        current_page: HPDF_Page,
        has_offset: &mut bool,
    ) -> Result<(), String> {
        unsafe {
            match content {
                StoryContent::Content(text) => {
                    // Font and size
                    let font = match (&render_properties.font_name, &render_properties.font_style) {
                        (Some(f_name), Some(f_style)) => {
                            font_lib.get_font(&f_name, &f_style).unwrap()
                        }
                        _ => {
                            return Err(format!(
                                "Cannot print text. Please set font name and style"
                            ))
                        }
                    };
                    HPDF_Page_SetFontAndSize(
                        current_page,
                        font,
                        render_properties.font_size.unwrap() as f32,
                    );

                    // Color
                    if let Some(color) = render_properties.fill_color {
                        set_fill_color(current_page, color);
                    }
                    if let Some(color) = render_properties.stroke_color {
                        set_stroke_color(current_page, color);
                    }

                    // Leading
                    let leading = render_properties.auto_leading.unwrap() / 100_f64
                        * render_properties.font_size.unwrap();
                    HPDF_Page_SetTextLeading(current_page, leading as f32);

                    if *has_offset == false {
                        *has_offset = true;
                        HPDF_Page_MoveToNextLine(current_page);
                    }

                    let bb = boundingbox(&self, parent_transform);
                    let mut text_remaining: &str = text;
                    while text_remaining.len() > 0 {
                        let pos = HPDF_Page_GetCurrentTextPos(current_page);
                        if (pos.y as f64) < bb.bottom {
                            break;
                        }
                        let available_chars = HPDF_Page_MeasureText(
                            current_page,
                            CString::new(text_remaining.clone()).unwrap().as_ptr(),
                            (bb.right as f32) - pos.x,
                            HPDF_TRUE,
                            ptr::null_mut(),
                        );
                        if available_chars > 0 {
                            let (text_to_print, remaning) =
                                text_remaining.split_at(available_chars as usize);
                            text_remaining = remaning;
                            HPDF_Page_ShowText(
                                current_page,
                                CString::new(text_to_print).unwrap().as_ptr(),
                            );
                            println!("{}: {:?}, {:?}", text_to_print, pos.x, pos.y);
                        } else {
                            HPDF_Page_MoveToNextLine(current_page);
                        }
                    }
                }
                StoryContent::Br => {
                    HPDF_Page_MoveToNextLine(current_page);
                }
                _ => {}
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
