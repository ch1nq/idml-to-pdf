    use printpdf::{PdfDocumentReference, IndirectFontRef};
// use regex;
use crate::idml_parser::IDMLResources;
// use crate::idml_parser::fonts_parser;
// use std::fs::File;


pub fn load_all_fonts(idml_resources: &IDMLResources, pdf_doc: &PdfDocumentReference) -> Result<Vec<IndirectFontRef>, printpdf::Error> {
    
    // Loop over every font and load it
    let font_refs = idml_resources
    .fonts().font_families().into_iter().flat_map(|family|
        family.fonts().iter().map(|font| 
            load_font_from_id(idml_resources, pdf_doc, font.id()).unwrap()
        )
    ).collect();
    
    Ok(font_refs)
}

pub fn load_font_from_id(idml_resources: &IDMLResources, pdf_doc: &PdfDocumentReference, id: &String) -> Result<IndirectFontRef, printpdf::Error> {
    // TODO: Actually load in the font from the id
    // let font = idml_resources.fonts().font_from_id(id);
    // let font = pdf_doc.add_external_font(File::open("assets/fonts/RobotoMedium.ttf").unwrap()).unwrap();

    // For now, just add a default font 
    Ok(pdf_doc.add_builtin_font(printpdf::BuiltinFont::HelveticaBold)?)
}