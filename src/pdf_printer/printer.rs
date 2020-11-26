use std::fs::File;
use std::io::BufWriter;
use printpdf::*;
use printpdf::indices::{PdfLayerIndex, PdfPageIndex};

use crate::idml_parser::package_parser::IDMLPackage;
use crate::idml_parser::spread_parser::*;

use super::transforms::{self, Transform};
use super::polygon::RenderPolygon;

pub struct PDFPrinter {
    idml_package: IDMLPackage,
    pdf_doc: PdfDocumentReference,
}

impl PDFPrinter {
    pub fn new(idml_package:IDMLPackage) -> Result<PDFPrinter, Error> {
        
        let doc = PdfDocument::empty("PDF_Document_title");
        
        let printer = PDFPrinter {
            idml_package: idml_package,
            pdf_doc: doc,

        };

        printer.render_pdf()?;

        Ok(printer)
    }

    fn render_pdf(&self) -> Result<(), Error> {
        for spread in self.idml_package.spreads().into_iter() {
            self.render_spread(spread).expect(format!("Failed to render spread {:?}", spread).as_str());
        } 

        Ok(())
    }
    
    fn render_spread(&self, spread: &Spread) -> Result<(), Error> {

        // References to page and layer index 
        // Udated everytime a new page is created
        let mut current_page_index = None;
        let mut current_layer_index = None;
        
        // Make transformation matrices
        let invert_y_axis = transforms::from_values(1_f64,0_f64,0_f64,-1_f64,0_f64,0_f64);
        let spread_transform = transforms::from_vec(spread.item_transform()).combine_with(&invert_y_axis);
        let mut page_transform = transforms::identity();

        for content in spread.contents().into_iter() {
            self.render_spread_content(content, &spread_transform, &mut page_transform, &mut current_page_index, &mut current_layer_index)?;
        }

        Ok(())
    }
    
    fn render_spread_content(&self, content: &SpreadContent, spread_transform: &Transform, page_transform: &mut Transform, page_index: &mut Option<PdfPageIndex>, layer_index: &mut Option<PdfLayerIndex>) 
        -> Result<(), Error> 
    {
        match content {
            SpreadContent::Page(p) => {
                // Update page tranformation matrix
                *page_transform = transforms::from_vec(p.item_transform()).reverse();
                *page_transform = page_transform.combine_with(spread_transform);

                // Make a new page
                let (page_id, layer_id) = self.render_blank_page(p, page_transform)
                    .expect(format!("Failed to render page '{}'", p.id()).as_str());

                // Update the current page and layer index
                *page_index = Some(page_id);
                *layer_index = Some(layer_id);

            }
            SpreadContent::Rectangle(r) => { 
                r.render(page_transform, &self.pdf_doc, &self.idml_package.resources(), page_index, layer_index)
                .expect(format!("Failed to render rectangle '{}'", r.id()).as_str())
            }
            SpreadContent::Polygon(p) => { 
                p.render(page_transform, &self.pdf_doc, &self.idml_package.resources(), page_index, layer_index)
                .expect(format!("Failed to render polygon '{}'", p.id()).as_str())
            }
            SpreadContent::TextFrame(t) => { 
                t.render(page_transform, &self.pdf_doc, &self.idml_package.resources(), page_index, layer_index)
                .expect(format!("Failed to render textframe '{}'", t.id()).as_str())
            }
            SpreadContent::Oval(o) => { 
                o.render(page_transform, &self.pdf_doc, &self.idml_package.resources(), page_index, layer_index)
                .expect(format!("Failed to render oval '{}'", o.id()).as_str())
            }
            _ => {}
        }
            
        Ok(())
    }

    fn render_blank_page(&self, page: &Page, page_transform: &Transform) -> Result<(PdfPageIndex,PdfLayerIndex), String> {
        if let [y1, x1, y2, x2] = page.geometric_bounds().as_slice() {
            // Top left and bottom right corners of page
            let point1 = page_transform.apply_to_point(x1,y1);
            let point2 = page_transform.apply_to_point(x2,y2);

            // Generate the page in the PDF
            let ids = self.pdf_doc.add_page(Mm::from(Pt(point2[0]-point1[0])), Mm::from(Pt(point2[1]-point1[1])), "New page");

            Ok(ids)
        } else {
            Err(format!("Geometric bounds '{:?}' did not match [y1, x1, y2, x2]", page.geometric_bounds().as_slice()))
        }
    }

    pub fn save_pdf(self, path: &str) -> Result<(), Error> {
        self.pdf_doc.save(&mut BufWriter::new(File::create(path).unwrap()))?;   
        Ok(())
    }
}
