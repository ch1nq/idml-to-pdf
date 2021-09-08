use idml_to_pdf::idml_parser::idml_package::IDMLPackage;
use idml_to_pdf::pdf_printer::PDFPrinter;
use std::path::PathBuf;

/// Macro that prints how long an operation took to run.
/// Multiple lines of code can be given as the second argument.
/// The last line should not contain a semicolon.
/// Example: ```
///     time_it!( "My set of operations",
///         let x = 2;
///         foo(x);
///         let y = x+2 // <-- No semicolon on last line
///     );
///
///     // Output: "My set of operations: 34ms"
/// ```
macro_rules! time_it {
    ($context:literal, $($s:stmt);+) => {
        let timer = std::time::Instant::now();
        $(
            $s
        )*
        println!("{}: {:?}", $context, timer.elapsed());
    }
}

fn main() {
    // Argument 1: IDML file path
    let file_path = PathBuf::from(std::env::args().nth(1).expect("No path to IDML file given"));

    // If no output path specified, just make a PDF in the same
    // folder as the IDML file
    let mut default_pdf_path = PathBuf::from(&file_path);
    default_pdf_path.set_extension("pdf");

    // Argument 2: IDML file path
    let pdf_path = std::env::args().nth(2).unwrap_or(
        default_pdf_path.to_str().unwrap().to_string(), // Default
    );

    // Argument 3: Font directory path
    let font_dir = match std::env::args().nth(3) {
        Some(path) => Some(PathBuf::from(&path)),
        None => None,
    };

    std::process::exit(real_main(file_path, &pdf_path, font_dir).unwrap());
}

/// Generates a PDF from a given IDML file
pub fn real_main(
    file_path: PathBuf,
    pdf_path: &str,
    font_dir: Option<PathBuf>,
) -> Result<i32, std::io::Error> {
    // Parse contents of a directory into an IDML Package
    time_it!( "Parsing IDML", 
        let idml_package = IDMLPackage::from_path(&file_path)?
    );

    // Make IDML package into a PDF document
    time_it!( "Making the PDF",
        let pdf_printer = PDFPrinter::new(&idml_package, &font_dir).unwrap();
        pdf_printer.render_pdf().unwrap()
    );

    // Save the PDF document
    time_it!("Saving the PDF", pdf_printer.save_pdf(&pdf_path).unwrap());

    return Ok(1);
}
