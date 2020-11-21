use std::fs::File;
use std::io::BufWriter;
use printpdf::*;
use crate::idml_parser::package_parser::IDMLPackage;
use crate::idml_parser::spread_parser::*;

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

    pub fn save_pdf(self, path: &str) -> Result<(), Error> {
        
        // println!("{:#?}", self.idml_package.master_spreads());
        
        let master_spreads = self.idml_package.master_spreads();
        
        for spread in master_spreads.into_iter() {
            self.render_spread(spread);
        } 
        
        self.pdf_doc.save(&mut BufWriter::new(File::create(path).unwrap()))?;

        Ok(())
    }

    fn render_spread(&self, spread: &Spread) {
        let pages = spread.pages();

        for page in pages.into_iter() {
            if let Some(p) = page { 
                self.render_blank_page(p) 
            }
        }
    }

    fn render_blank_page(&self, page: &Page) {

        if let [y1, x1, y2, x2] = page.geometric_bounds().as_slice() {
            println!("({} {}) ({} {})", y1, x1, y2, x2);
        } else {
            println!("Match refuted.");
        }
        
    }
}

