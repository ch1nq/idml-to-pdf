use idml_to_pdf::idml_parser::package_parser::IDMLPackage;
use idml_to_pdf::pdf_printer::printer::PDFPrinter;
use idml_to_pdf::decompressor;

fn main() {
    std::process::exit(real_main().unwrap());
}

fn real_main() -> Result<i32,std::io::Error> {
    // Parse arguments
    // let args: Vec<_> = std::env::args().collect();

    let file_path = std::env::args().nth(1).expect("No path to IDML file given");
    let pdf_path = std::env::args().nth(2).expect("No path to PDF file given");
    let preserve_idml_dir = std::env::args().nth(3);

    
    // Decrompress idml file into a directory
    let idml_dir = decompressor::decompress_idml(&file_path).unwrap();
    
    
    // Get a IDML Package object 
    let idml_package = IDMLPackage::from_dir(&idml_dir)?;
    
    // Pass it to printer object
    let pdf_printer = PDFPrinter::new(idml_package).unwrap();
    
    // Print a pdf to specified path
    pdf_printer.save_pdf(&pdf_path).unwrap();
    
    // Remove idml directory
    if preserve_idml_dir.is_some() {
        println!("{:?}", idml_dir);
    } else {
        std::fs::remove_dir_all(idml_dir)?;
    }

    return Ok(1);
}
