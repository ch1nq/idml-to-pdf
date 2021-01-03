use idml_to_pdf::idml_parser::IDMLPackage;
use idml_to_pdf::pdf_printer::PDFPrinter;
use idml_to_pdf::decompressor;
use std::path::PathBuf;
use std::time::{Duration, Instant};

fn main() {
    std::process::exit(real_main().unwrap());
}

fn real_main() -> Result<i32,std::io::Error> {
    // Argument 1: IDML file path
    let file_path = std::env::args().nth(1)
        .expect("No path to IDML file given");
    
    // If no output path specified, just make a PDF in the same 
    // folder as the IDML file 
    let mut default_pdf_path = PathBuf::from(&file_path);
    default_pdf_path.set_extension("pdf");
    
    // Argument 2: IDML file path
    let pdf_path = std::env::args().nth(2).unwrap_or(
        default_pdf_path.to_str().unwrap().to_string() // Default
    );

    // Argument 3: Font directory path
    let font_dir = match std::env::args().nth(3) {
        Some(path) => Some(PathBuf::from(&path)),
        None => None
    };

    // Argument 4: If the temporary folder with the extracted IDML data, should be kept
    // If no argument is provided, the folder will be deleted 
    let preserve_idml_dir = std::env::args().nth(4);

    // Decrompress idml file into a directory
    let idml_dir = decompressor::decompress_idml(&file_path).unwrap();
    
    // Get a IDML Package object 
    let start = Instant::now();
    let idml_package = IDMLPackage::from_dir(&idml_dir)?;
    let duration = start.elapsed();
    println!("Parsing took {:?}", duration);
    
    // Pass it to printer object
    let start = Instant::now();
    let pdf_printer = PDFPrinter::new(idml_package, font_dir).unwrap();
    let duration = start.elapsed();
    println!("Making the PDF took {:?}", duration);
    
    // Print a pdf to specified path
    let start = Instant::now();
    pdf_printer.save_pdf(&pdf_path).unwrap();
    let duration = start.elapsed();
    println!("Saving the PDF took {:?}", duration);
    
    // Remove idml directory
    if preserve_idml_dir.is_some() {
        println!("{:?}", idml_dir);
    } else {
        std::fs::remove_dir_all(idml_dir)?;
    }

    return Ok(1);
}
