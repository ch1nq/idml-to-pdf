use crate::idml_parser::{
    spread_parser::*, story_parser::*, styles::commom_text_properties::*, styles::*, IDMLPackage,
    IDMLResources,
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

    fn with_attributes(&mut self, ctp: &impl CommonTextProperties) -> &mut Self {
        // for (key, value) in ctp.ctp_fields() {
        for field in ctp.ctp_fields().iter() {
            // match (key, value) {
            match field {
                // (CTPKey::AutoLeading, CTPValue::F64(value)) => self.auto_leading = Some(*value),
                // (CTPKey::PointSize, CTPValue::F64(value)) => self.font_size = Some(*value),
                // (CTPKey::FontStyle, CTPValue::String(value)) => self.font_style = Some(value.clone()),
                // (CTPKey::FillColor, CTPValue::String(value)) => self.set_fill_color(value.clone()),
                // (CTPKey::StrokeColor, CTPValue::String(value)) => self.set_stroke_color(value.clone()),
                _ => {}
            }
        }

        if let Some(properties) = ctp.properties() {
            if let Some(font) = properties.applied_font() {
                self.font_name = Some(font.clone());
            }
        }

        self
    }

    fn set_stroke_color(&mut self, color_id: String) {
        let color = match color_manager::color_from_id(self.idml_resources, &color_id) {
            Ok(c) => Some(c),
            Err(ColorError::ColorNotImplemented) => None,
            Err(_) => None,
        };
        self.stroke_color = color;
    }

    fn set_fill_color(&mut self, color_id: String) {
        let color = match color_manager::color_from_id(self.idml_resources, &color_id) {
            Ok(c) => Some(c),
            Err(ColorError::ColorNotImplemented) => None,
            Err(_) => None,
        };
        self.fill_color = color;
    }
}

impl Polygon {
    pub fn render_story(
        &self,
        idml_package: &IDMLPackage,
        parent_transform: &mut Transform,
        font_lib: &FontLibrary,
        current_page: HPDF_Page,
    ) -> Result<(), String> {
        if let Some(story_id) = self.parent_story() {
            if let Some(story) = idml_package.stories().get(story_id) {
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
        // if let Some(CTPValue::String(style_id)) = p_style.ctp_fields().get(&CTPKey::AppliedCharacterStyle) {
        //     if let Some(style) = idml_resources.styles().paragraph_style_from_id(style_id) {
        //         render_properties.with_attributes(&style);
        //     }
        // }

        // Apply local paragraph formats
        render_properties.with_attributes(p_style);

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
        // if let Some(CTPValue::String(style_id)) = c_style.ctp_fields().get(&CTPKey::AppliedCharacterStyle) {
        //     if let Some(style) = idml_resources.styles().character_style_from_id(style_id) {
        //         render_properties.with_attributes(&style);
        //     }
        // }

        // Apply local character formats
        render_properties.with_attributes(c_style);

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
