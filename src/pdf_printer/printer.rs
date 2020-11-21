use std::fs::File;
use std::io::BufWriter;
use printpdf::*;
// use crate::idml_parser::IDMLPackage;
use crate::idml_parser::package_parser::IDMLPackage;

pub struct PDFPrinter {
    idml_package: IDMLPackage,
    pdf_doc: PdfDocumentReference
}

impl PDFPrinter {
    pub fn new(idml_package:IDMLPackage) -> Result<PDFPrinter, ()> {
        // let (doc, page1, layer1) = PdfDocument::new("PDF_Document_title", Mm(247.0), Mm(210.0), "Layer 1");
        // let (page2, layer1) = doc.add_page(Mm(10.0), Mm(250.0),"Page 2, Layer 1");
        let (doc, _, _) = PdfDocument::new("PDF_Document_title", Mm(247.0), Mm(210.0), "Layer 1");
        let (_, _) = doc.add_page(Mm(10.0), Mm(250.0),"Page 2, Layer 1");

        Ok(PDFPrinter {
            idml_package: idml_package,
            pdf_doc: doc
        })
    }

    pub fn print_pdf(self, path: &str) -> Result<(), Error> {
        
        println!("{:#?}", self.idml_package);

        self.pdf_doc.save(&mut BufWriter::new(File::create(path).unwrap()))?;

        Ok(())
    }
}

