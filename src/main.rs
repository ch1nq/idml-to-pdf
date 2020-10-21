mod decompressor;
mod parser;

extern crate printpdf;
extern crate xml;

use std::fs::remove_dir_all;
use parser::IDMLPackage;

fn main() {
    std::process::exit(real_main().unwrap());
}

fn real_main() -> Result<i32,std::io::Error> {
    // Parse arguments
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return Ok(0);
    }
    let file_path = &*args[1].to_string();
    
    // Decrompress idml file into a directory
    let idml_dir = decompressor::decompress_idml(file_path).unwrap();
    
    let idml_package = IDMLPackage::from_dir(&idml_dir);

    println!("{:#?}", idml_package);

    // Remove idml directory
    //remove_dir_all(idml_dir)?;

    return Ok(1);
}
