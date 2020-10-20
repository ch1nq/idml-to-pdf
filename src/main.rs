mod decompressor;

extern crate printpdf;
extern crate xml;

use std::fs::{File, remove_dir_all};
use std::io::{BufReader};
use std::path::{Path};
use xml::reader::{EventReader, XmlEvent};

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

fn read_idml(idml_dir:&Path) {
    let file = File::open("files/test-1/Spreads/Spread_uce.xml").unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                if name.to_string() == "Rectangle" {
                    let attributes:Vec<Vec<String>> = attributes.into_iter().map(|a| {vec!(a.name.to_string(), a.value)}).collect();
                    println!("{}: {:#?}", name, attributes);
                }
            }
            // Ok(XmlEvent::EndElement {..}) => {}
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
}
