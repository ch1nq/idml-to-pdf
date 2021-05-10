pub mod color_manager;
mod font_manager;
mod page_items;
mod pdf_utils;
mod transforms;

use crate::idml_parser::spread_parser::*;
use crate::idml_parser::IDMLPackage;
use font_manager::FontLibrary;
use libharu_sys::*;
use page_items::polygon::RenderPolygon;
use std::ffi::CString;
use std::path::PathBuf;
use std::ptr;
use transforms::Transform;

extern "C" fn error_handler(error_no: HPDF_STATUS, detail_no: HPDF_STATUS, user_data: HPDF_HANDLE) {
    println!(
        "ERROR: error_no={:#X}, detail_no={:#X}, user_data={:?}",
        error_no, detail_no, user_data
    );
}

macro_rules! cstring {
    ($fmt:expr) => {
        CString::new($fmt).unwrap()
    };
}

pub struct PDFPrinter<'a> {
    idml_package: &'a IDMLPackage,
    font_lib: FontLibrary<'a>,
    pdf_doc: HPDF_Doc,
}

impl<'a> PDFPrinter<'a> {
    pub fn new(
        idml_package: &'a IDMLPackage,
        resource_dir: &'a Option<PathBuf>,
    ) -> Result<PDFPrinter<'a>, String> {
        unsafe {
            let pdf_doc = HPDF_New(error_handler, ptr::null_mut());
            if pdf_doc == ptr::null_mut() {
                return Err(format!("error: cannot create PdfDoc object"));
            }
            let font_lib =
                FontLibrary::new(&idml_package.resources(), pdf_doc, resource_dir).unwrap();
            let printer = PDFPrinter {
                idml_package,
                font_lib,
                pdf_doc,
            };
            Ok(printer)
        }
    }

    pub fn render_pdf(&self) -> Result<(), String> {
        // Render each spread
        for spread in self.idml_package.spreads() {
            self.render_spread(spread)
                .expect(format!("Failed to render spread {:?}", spread).as_str());
        }
        Ok(())
    }

    fn render_spread(&self, spread: &Spread) -> Result<(), String> {
        // References to page and layer index
        // Udated everytime a new page is created
        let mut current_page = None;

        // Make transformation matrices
        let invert_y_axis = transforms::from_values(1_f64, 0_f64, 0_f64, -1_f64, 0_f64, 0_f64);
        let spread_transform =
            transforms::from_vec(spread.item_transform()).combine_with(&invert_y_axis);
        // let spread_transform = transforms::from_vec(spread.item_transform());

        let mut page_transform = transforms::identity();

        for content in spread.contents() {
            self.render_spread_content(
                content,
                &spread_transform,
                &mut page_transform,
                &mut current_page,
            )?;
        }

        Ok(())
    }

    fn render_spread_content(
        &self,
        content: &SpreadContent,
        spread_transform: &Transform,
        page_transform: &mut Transform,
        current_page: &mut Option<HPDF_Page>,
    ) -> Result<(), String> {
        match content {
            SpreadContent::Page(p) => {
                // Update page tranformation matrix
                *page_transform = transforms::from_vec(p.item_transform()).reverse();
                *page_transform = page_transform.combine_with(spread_transform);

                // Make a new page
                let page = self
                    .render_blank_page(p, page_transform)
                    .expect(format!("Failed to render page '{}'", p.id()).as_str());

                // Update the current page reference
                *current_page = Some(page);
            }
            SpreadContent::Rectangle(r) => {
                r.render(
                    page_transform,
                    &self.idml_package.resources(),
                    current_page.expect("No page found"),
                )
                .expect(format!("Failed to render rectangle '{}'", r.id()).as_str());
            }
            SpreadContent::Polygon(p) => {
                p.render(
                    page_transform,
                    &self.idml_package.resources(),
                    current_page.expect("No page found"),
                )
                .expect(format!("Failed to render polygon '{}'", p.id()).as_str());
            }
            SpreadContent::TextFrame(t) => {
                t.render(
                    page_transform,
                    &self.idml_package.resources(),
                    current_page.expect("No page found"),
                )
                .expect(format!("Failed to render textframe '{}'", t.id()).as_str());
                t.render_story(
                    &self.idml_package,
                    page_transform,
                    self.pdf_doc,
                    &self.font_lib,
                    current_page.expect("No page found"),
                )
                .expect(format!("Failed to render story of textframe '{}'", t.id()).as_str());
            }
            SpreadContent::Oval(o) => {
                o.render(
                    page_transform,
                    &self.idml_package.resources(),
                    current_page.expect("No page found"),
                )
                .expect(format!("Failed to render oval '{}'", o.id()).as_str());
            }
            _ => {}
        }

        Ok(())
    }

    fn render_blank_page(
        &self,
        page: &Page,
        page_transform: &mut Transform,
    ) -> Result<HPDF_Page, String> {
        if let [y1, x1, y2, x2] = page.geometric_bounds().as_slice() {
            // Top left and bottom right corners of page
            let point1 = page_transform.apply_to_point(x1, y1);
            let point2 = page_transform.apply_to_point(x2, y2);
            let width = point2[0] - point1[0];
            let height = point2[1] - point1[1];
            let transpose = &transforms::from_values(1_f64, 0_f64, 0_f64, 1_f64, 0_f64, -height);
            *page_transform = page_transform.combine_with(transpose);

            unsafe {
                // Generate the page in the PDF
                let current_page = HPDF_AddPage(self.pdf_doc);
                HPDF_Page_SetWidth(current_page, width.abs() as f32);
                HPDF_Page_SetHeight(current_page, height.abs() as f32);
                Ok(current_page)
            }
        } else {
            Err(format!(
                "Geometric bounds '{:?}' did not match [y1, x1, y2, x2]",
                page.geometric_bounds().as_slice()
            ))
        }
    }

    pub fn save_pdf(self, path: &str) -> Result<(), String> {
        unsafe {
            let fname = cstring!(path);
            HPDF_SaveToFile(self.pdf_doc, fname.as_ptr());
            HPDF_Free(self.pdf_doc);
        }
        Ok(())
    }
}
