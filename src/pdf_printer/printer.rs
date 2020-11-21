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
    pub fn new(idml_package:IDMLPackage) -> Result<PDFPrinter, Error> {
        // let (doc, page1, layer1) = PdfDocument::new("PDF_Document_title", Mm(247.0), Mm(210.0), "Layer 1");
        // let (page2, layer1) = doc.add_page(Mm(10.0), Mm(250.0),"Page 2, Layer 1");
        let doc = PdfDocument::empty("PDF_Document_title");
        
        let mut printer = PDFPrinter {
            idml_package: idml_package,
            pdf_doc: doc
        };

        printer.render_pdf();

        Ok(printer)
    }

    fn render_pdf(&self) -> Result<(), Error> {
        for spread in self.idml_package.spreads().into_iter() {
            self.render_spread(spread).expect(format!("Failed to render spread {:?}", spread).as_str());
        } 

        Ok(())
    }
    
    fn render_spread(&self, spread: &Spread) -> Result<(), Error> {
        for page in spread.pages().into_iter() {
            if let Some(p) = page { 
                self.render_blank_page(p).expect(format!("Failed to render page {:?}", p).as_str());
            }
        }

        Ok(())
    }
    
    fn render_blank_page(&self, page: &Page) -> Result<(), String> {
        
        if let [y1, x1, y2, x2] = page.geometric_bounds().as_slice() {
            println!("({} {}) ({} {})", y1, x1, y2, x2);
            
            let (_, _) = self.pdf_doc.add_page(Mm(x2-x1), Mm(y2-y1), "New page");
            
        } else {
            return Err(format!("Geometric bounds '{:?}' did not match [y1, x1, y2, x2]", page.geometric_bounds().as_slice()));
        }
        
        Ok(())
    }

    pub fn save_pdf(self, path: &str) -> Result<(), Error> {
        
        // println!("{:#?}", self.idml_package.master_spreads());
                
        self.pdf_doc.save(&mut BufWriter::new(File::create(path).unwrap()))?;
        
        Ok(())
    }
}

