mod decompressor;
mod parser;
mod printer;
mod story_parser;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_xml_rs;
extern crate printpdf;

// use std::fs::remove_dir_all;
// use parser::IDMLPackage;
// use printer::PDFPrinter;
use std::path::PathBuf;

fn main() {
    std::process::exit(real_main().unwrap());
}

fn real_main() -> Result<i32,std::io::Error> {
    // Parse arguments
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <idml> <output>", args[0]);
        return Ok(0);
    }
    let file_path = &*args[1].to_string();
    let pdf_path = &*args[2].to_string();
    
    // Decrompress idml file into a directory
    let idml_dir = decompressor::decompress_idml(file_path).unwrap();
    
    // let mut test_story_path = PathBuf::from(idml_dir);
    // test_story_path.push("Stories/Story_u2a1.xml");

    let story = story_parser::parse_story_from_path(&test_story_path);
    println!("{:#?}", story);

    // // Get a IDML Package object 
    // let idml_package = IDMLPackage::from_dir(&idml_dir);

    // println!("{:#?}", idml_package);

    // // Pass it to printer object
    // let pdf_printer = PDFPrinter::new(idml_package).unwrap();

    // // Print a pdf to specified path
    // pdf_printer.print_pdf(pdf_path).unwrap();

    // Remove idml directory
    //remove_dir_all(idml_dir)?;

    return Ok(1);
}
