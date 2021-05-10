use crate::idml_parser::fonts_parser::Font;
use crate::idml_parser::IDMLResources;
use dirs;
use libharu_sys::*;
use std::cell::Cell;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::CString;
use std::fs;
use std::iter::FromIterator;
use std::marker::Copy;
use std::path::PathBuf;
use std::ptr;

#[derive(Debug, PartialEq, Hash, Eq, Copy, Clone)]
pub struct FontId<'a> {
    font_name: &'a str,
    font_style: &'a str,
}

#[derive(Debug, Copy, Clone)]
pub enum FontStatus<'a> {
    Uninitialized(&'a Font),
    Initialized(HPDF_Font),
}

#[derive(Debug)]
pub struct FontLibrary<'a> {
    resource_dir: &'a Option<PathBuf>,
    pdf_doc: HPDF_Doc,
    fonts: HashMap<FontId<'a>, Cell<FontStatus<'a>>>,
}

impl<'a> FontLibrary<'a> {
    pub fn get_font(&self, font_name: &str, font_style: &str) -> Result<HPDF_Font, String> {
        let id = FontId {
            font_name,
            font_style,
        };
        match self.fonts.get(&id) {
            Some(cell) => match cell.get() {
                FontStatus::Uninitialized(idml_font) => {
                    if let Ok(pdf_font) = self.load_font_from_id(idml_font.post_script_name()) {
                        cell.set(FontStatus::Initialized(pdf_font));
                        Ok(pdf_font)
                    } else if let Ok(pdf_font) =
                        self.load_font_from_id(idml_font.full_name_native())
                    {
                        cell.set(FontStatus::Initialized(pdf_font));
                        Ok(pdf_font)
                    } else {
                        println!("No font matched: {:#?}", id);
                        // Return fallback font
                        unsafe {
                            let font = HPDF_GetFont(
                                self.pdf_doc,
                                CString::new("ZapfDingbats").unwrap().as_ptr(),
                                ptr::null_mut(),
                            );
                            Ok(font)
                        }
                    }
                }
                FontStatus::Initialized(font) => Ok(font),
            },
            None => Err(format!(
                "Font not declared in resources: {} {}",
                font_name, font_style
            )
            .to_string()),
        }
    }

    pub fn new(
        idml_resources: &'a IDMLResources,
        pdf_doc: HPDF_Doc,
        resource_dir: &'a Option<PathBuf>,
    ) -> Result<FontLibrary<'a>, String> {
        // Load every font from every font-family in the IDML resources
        let font_refs = idml_resources
            .fonts()
            .font_families()
            .into_iter()
            .flat_map(|family| {
                family.fonts().iter().map(|font| {
                    (
                        FontId {
                            font_name: font.font_family(),
                            font_style: font.font_style_name(),
                        },
                        Cell::new(FontStatus::Uninitialized(font)),
                    )
                })
            });

        let font_lib = FontLibrary {
            resource_dir: resource_dir,
            pdf_doc,
            fonts: HashMap::from_iter(font_refs),
        };

        Ok(font_lib)
    }
    fn load_font_from_id(&self, id: &str) -> Result<HPDF_Font, String> {
        let mut font_lookup = vec![];
        // Search in provided font directory
        if let Some(font_dir) = self.resource_dir {
            font_lookup.append(&mut find_font_in_dir(id, &font_dir));
        }
        // Search in the OS font directory
        if let Some(font_dir) = dirs::font_dir() {
            font_lookup.append(&mut find_font_in_dir(id, &font_dir));
        }
        unsafe {
            match &font_lookup[..] {
                [] => Err(format!("No font matched: {}", id)),
                [font_path] => {
                    let font_name = HPDF_LoadTTFontFromFile(
                        self.pdf_doc,
                        CString::new(font_path.to_str().unwrap()).unwrap().as_ptr(),
                        HPDF_TRUE,
                    );
                    // println!("{:?}: {:#?}", id, font_path);
                    let font = HPDF_GetFont(self.pdf_doc, font_name, ptr::null_mut());
                    Ok(font)
                }
                [font_path, ..] => {
                    let font_name = HPDF_LoadTTFontFromFile(
                        self.pdf_doc,
                        CString::new(font_path.to_str().unwrap()).unwrap().as_ptr(),
                        HPDF_FALSE,
                    );
                    let font = HPDF_GetFont(self.pdf_doc, font_name, ptr::null_mut());
                    Ok(font)
                }
            }
        }
    }
}

/// Get a list of paths to every file matching font_name in a given directory
fn find_font_in_dir(font_name: &str, dir: &PathBuf) -> Vec<PathBuf> {
    fs::read_dir(dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| {
            path.file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .contains(font_name)
        })
        .collect()
}
