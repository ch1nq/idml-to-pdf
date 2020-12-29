// use regex;
// use crate::idml_parser::fonts_parser;
// use std::fs::File;
use printpdf::{PdfDocumentReference, IndirectFontRef};
use crate::idml_parser::IDMLResources;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::cell::{RefCell};

#[derive(Debug)]
pub struct FontLibrary {
    // font_families: HashMap<String, FontFamily>,
    // font_families: HashMap<String, HashMap<String, IndirectFontRef>>,
    fonts: HashMap<(String, String), IndirectFontRef>,
    // current_font: RefCell<Option<IndirectFontRef>>
}

// #[derive(Debug)]
// pub struct FontFamily {
//     fonts: HashMap<String, HashMap<String, IndirectFontRef>>
// }

impl FontLibrary {

    // pub fn get_current_font(&self) -> Option<IndirectFontRef> {
    //     // Return a clone of the font reference to make sure, 
    //     // that a borrow and a mutable borrow never occurs at the same time 
    //     self.current_font.borrow().clone()
    // }

    // pub fn set_current_font(&self, name: &str, style: &str) 
    // -> Result<Option<IndirectFontRef>, String> {
    //     match self.font_from_idml_name_and_style(name, style) {
    //         Some(font) => {
    //             // Update the reference, we don't actually care about the previous value
    //             let _ = self.current_font.replace(Some(font.clone()));
    //             Ok(self.get_current_font())
    //         },
    //         // Let the caller know if their requested font doesn't exist 
    //         // - this likely is an error on their end
    //         _ => Err("Failed to set font".to_string())
    //     }
    // }

    pub fn font_from_idml_name_and_style(&self, name: &str, style: &str)
    -> Option<&IndirectFontRef> {
        println!("{} {}", name, style);
        self.fonts.get(&(name.to_string(), style.to_string()))
    }

    pub fn new(idml_resources: &IDMLResources, pdf_doc: & PdfDocumentReference) 
    -> Result<FontLibrary, printpdf::Error> {

        // let family_iter = idml_resources.fonts().font_families().into_iter();
        // let font_families = family_iter.map(|family| 
        //     (
        //         // Map family name to set of fonts
        //         family.name().unwrap_or("No family name".to_string()), 
        //         HashMap::from_iter(
        //             family.fonts().iter().map(|font| 
        //                 (
        //                     // Map font id to PDF font
        //                     font.id().to_string(), 
        //                     load_font_from_id(idml_resources, pdf_doc, font.id()).unwrap()
        //                 )
        //             )
        //         )
        //     )
        // );
        
        // let font_iter = family_iter.map(|family|
        //     family.fonts().iter()
        // );

        // let font_refs_from_id = font_iter.map(|font| 
        //     (font.id().to_string(), load_font_from_id(idml_resources, pdf_doc, font.id()).unwrap())
        // );
        // // Also map family name to the font
        // let font_refs_from_family_name = font_iter.map(|font| 
        //     (font.family_name().to_string(), load_font_from_id(idml_resources, pdf_doc, font.id()).unwrap())
        // );

        // Load every font from every font-family in the IDML resources
        let font_refs = idml_resources
        .fonts().font_families().into_iter().flat_map(|family|
            family.fonts().iter().map(|font| 
                (
                    (
                        font.font_family().as_ref().unwrap_or(&"".to_string()).to_string(), 
                        font.font_style_name().as_ref().unwrap_or(&"".to_string()).to_string()
                    ), 
                    load_font_from_id(idml_resources, pdf_doc, font.id()).unwrap()
                )
            )
        );


        let font_lib = FontLibrary {
        //    font_families: HashMap::from_iter(font_families),
           fonts: HashMap::from_iter(font_refs),
        //    current_font: RefCell::new(None)
        };

        Ok(font_lib)
    }
}

fn load_font_from_id(_idml_resources: &IDMLResources, pdf_doc: &PdfDocumentReference, _id: &String) 
-> Result<IndirectFontRef, printpdf::Error> {
    // TODO: Actually load in the font from the id
    // let font = idml_resources.fonts().font_from_id(id);
    // let font = pdf_doc.add_external_font(File::open("assets/fonts/RobotoMedium.ttf").unwrap()).unwrap();
    
    let font = pdf_doc.add_external_font(std::fs::File::open("/Library/Fonts/Arial Unicode.ttf").unwrap()).unwrap();
    Ok(font)                    
    
    // For now, just add a default font 
    // Ok(pdf_doc.add_builtin_font(printpdf::BuiltinFont::HelveticaBold)?)
}
