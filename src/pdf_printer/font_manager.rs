// use regex;
// use crate::idml_parser::fonts_parser;
// use std::fs::File;
use printpdf::{PdfDocumentReference, IndirectFontRef};
use crate::idml_parser::IDMLResources;
use std::collections::HashMap;
use std::iter::FromIterator;

pub struct FontLibrary {
    fonts: HashMap<String, IndirectFontRef>
}

impl FontLibrary {
    pub fn idml_id_to_font(&self, id: &String)
    -> &Option<&IndirectFontRef> {
        //&self.fonts.get(id)
        &None
    }

    pub fn new(idml_resources: &IDMLResources, pdf_doc: &PdfDocumentReference) 
    -> Result<FontLibrary, printpdf::Error> {
        
        // Loop over every font and load it
        let font_refs = idml_resources
        .fonts().font_families().into_iter().flat_map(|family|
            family.fonts().iter().map(|font| 
                (font.id().to_string(), load_font_from_id(idml_resources, pdf_doc, font.id()).unwrap())
            )
        );

        let font_lib = FontLibrary {
           fonts: HashMap::from_iter(font_refs)
        };
        
        Ok(font_lib)
    }
}

fn load_font_from_id(idml_resources: &IDMLResources, pdf_doc: &PdfDocumentReference, id: &String) 
-> Result<IndirectFontRef, printpdf::Error> {
    // TODO: Actually load in the font from the id
    // let font = idml_resources.fonts().font_from_id(id);
    // let font = pdf_doc.add_external_font(File::open("assets/fonts/RobotoMedium.ttf").unwrap()).unwrap();

    // For now, just add a default font 
    Ok(pdf_doc.add_builtin_font(printpdf::BuiltinFont::HelveticaBold)?)
}
