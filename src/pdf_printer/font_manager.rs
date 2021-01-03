use printpdf::{PdfDocumentReference, IndirectFontRef};
use crate::idml_parser::IDMLResources;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::cell::{RefCell};
use dirs;
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub struct FontLibrary {
    fonts: HashMap<(String, String), IndirectFontRef>,
}

impl FontLibrary {

    pub fn font_from_idml_name_and_style(&self, name: &str, style: &str)
    -> Option<&IndirectFontRef> {
        println!("{} {}", name, style);
        self.fonts.get(&(name.to_string(), style.to_string()))
    }

    pub fn new(idml_resources: &IDMLResources, pdf_doc: & PdfDocumentReference, resource_dir: &Option<PathBuf>) 
    -> Result<FontLibrary, printpdf::Error> {

        // Load every font from every font-family in the IDML resources
        let font_refs = idml_resources
        .fonts().font_families().into_iter().flat_map(|family|
            family.fonts().iter().map(|font| 
                (
                    (
                        font.font_family().to_string(), 
                        font.font_style_name().to_string()
                    ), 
                    load_font_from_id(resource_dir, pdf_doc, font.post_script_name()).unwrap()
                )
            )
        );

        let font_lib = FontLibrary {
           fonts: HashMap::from_iter(font_refs),
        };

        Ok(font_lib)
    }
}

fn load_font_from_id(resource_dir: &Option<PathBuf>, pdf_doc: &PdfDocumentReference, id: &String) 
-> Result<IndirectFontRef, String> {

    /// Get a list of paths to every file matching font_name in a given directory 
    fn find_font_in_dir(font_name: &String, dir: &PathBuf) -> Vec<PathBuf> {
        fs::read_dir(dir).unwrap()        
            .map(|entry| entry.unwrap().path())
            .filter(|path| path.file_stem().unwrap().to_str().unwrap().contains(font_name))
            .collect()
    }

    let mut font_lookup = vec!();

    // Search in provided font directory
    if let Some(font_dir) = resource_dir {
        font_lookup.append(&mut find_font_in_dir(id, &font_dir));
    }
    
    // // Search in the OS font directory
    if let Some(font_dir) = dirs::font_dir() {
        font_lookup.append(&mut find_font_in_dir(id, &font_dir));
    }

    match &font_lookup[..] {
        // [] => Err("No font matched id".to_string()),
        [] => {
            println!("No font matched: {}", id);
            // let font = pdf_doc.add_external_font(std::fs::File::open("/Library/Fonts/Arial Unicode.ttf").unwrap()).unwrap();
            let font = pdf_doc.add_builtin_font(printpdf::BuiltinFont::TimesRoman).unwrap();
            Ok(font)
        },
        [font_path] => {
            let font = pdf_doc.add_external_font(std::fs::File::open(font_path).unwrap()).unwrap();
            Ok(font)                    
        },
        [font_path, ..] => {
            println!("Multiple fonts font matched: {}", id);
            // Err(format!("Multiple fonts matched: {}", id).to_string())
            let font = pdf_doc.add_external_font(std::fs::File::open(font_path).unwrap()).unwrap();
            Ok(font)
        }
    }   

}
