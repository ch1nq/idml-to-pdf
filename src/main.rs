mod decompressor;

extern crate printpdf;
extern crate xml;

use std::fs::remove_dir_all;

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

    // Remove idml directory
    remove_dir_all(idml_dir.clone())?;

    return Ok(1);
}
