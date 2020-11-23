use std::fs::File;
use std::io::BufWriter;
use printpdf::*;
use printpdf::indices::{PdfLayerIndex, PdfPageIndex};
use crate::idml_parser::package_parser::IDMLPackage;
use crate::idml_parser::spread_parser::*;
use ndarray::{arr2, Array2};

macro_rules! arr_identity {
    ($n:expr) => {
        arr2(&[[1_f64, 0_f64, 0_f64],
               [0_f64, 1_f64, 0_f64],
               [0_f64, 0_f64, 1_f64]])
    }
}

macro_rules! item_transform_matrix {
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr) => {
        arr2(&[[-$a.to_owned(), $b.to_owned(), 0_f64],
               [$c.to_owned(), -$d.to_owned(), 0_f64],
               [$e.to_owned(), $f.to_owned(), 1_f64]])        
    }
}

macro_rules! item_transform_matrix_from_opt_vec {
    ($opt_transform:expr) => {
        match $opt_transform {
            Some(transform) => match transform.as_slice() {
                [a,b,c,d,e,f] => item_transform_matrix!(a,b,c,d,e,f),
                _ => arr_identity!(3)
            }
            _ => arr_identity!(3)
        }
    }
}

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

        // They are updated everytime a new page is created
        let mut current_page_index = None;
        let mut current_layer_index = None;
        
        // Make transformation matrices
        let mut page_transform: Array2<f64> = arr_identity!(3);
        let spread_transform: Array2<f64> = item_transform_matrix_from_opt_vec!(spread.item_transform());
        
        for content in spread.contents().into_iter() {
            self.render_spread_content(content, &spread_transform, &mut page_transform, &mut current_page_index, &mut current_layer_index)?;
        }

        Ok(())
    }
    
    fn render_spread_content(&self, content: &SpreadContent, spread_transform: &Array2<f64>, page_transform: &mut Array2<f64>, page_index: &mut Option<PdfPageIndex>, layer_index: &mut Option<PdfLayerIndex>) 
        -> Result<(), Error> 
    {
        
        match content {
            SpreadContent::Page(p) => {
                // Update page tranformation matrix
                *page_transform = item_transform_matrix_from_opt_vec!(p.item_transform());
                *page_transform = page_transform.dot(spread_transform);

                // Make a new page
                let (page_id, layer_id) = self.render_blank_page(p, page_transform)
                    .expect(format!("Failed to render page '{}'", p.id()).as_str());

                // Update the current page and layer index
                *page_index = Some(page_id);
                *layer_index = Some(layer_id);

            }
            SpreadContent::Rectangle(r) => { 
                self.render_rectangle(r, page_transform, page_index, layer_index)
                .expect(format!("Failed to render rectangle '{}'", r.id()).as_str())
            }
            SpreadContent::Polygon(p) => { 
                self.render_polygon(p, page_transform, page_index, layer_index)
                .expect(format!("Failed to render polygon '{}'", p.id()).as_str())
            }
            SpreadContent::TextFrame(t) => { 
                self.render_textframe(t, page_transform, page_index, layer_index)
                .expect(format!("Failed to render textframe '{}'", t.id()).as_str())
            }
            SpreadContent::Oval(o) => { 
                self.render_oval(o, page_transform, page_index, layer_index)
                .expect(format!("Failed to render oval '{}'", o.id()).as_str())
            }
            _ => {}
        }
            
        Ok(())
    }
        
    fn render_blank_page(&self, page: &Page, page_transform: &Array2<f64>) -> Result<(PdfPageIndex,PdfLayerIndex), String> {

        if let [y1, x1, y2, x2] = page.geometric_bounds().as_slice() {

            let point1 = page_transform.dot(&arr2(&[[x1.to_owned()], [y1.to_owned()], [1_f64]]));
            let point2 = page_transform.dot(&arr2(&[[x2.to_owned()], [y2.to_owned()], [1_f64]]));

            // println!("{:#?}", point1);
            // println!("{:#?}", point2);
            
            let ids = self.pdf_doc.add_page(Mm::from(Pt(point2[[0,0]]-point1[[0,0]])), Mm::from(Pt(point2[[1,0]]-point1[[1,0]])), "New page");

            // println!("Page bounds: ({:#?}) ({:#?})", Mm::from(Pt(point2[[0,0]]-point1[[0,0]])), Mm::from(Pt(point2[[1,0]]-point1[[1,0]])));
            
            return Ok(ids)
        } else {
            return Err(format!("Geometric bounds '{:?}' did not match [y1, x1, y2, x2]", page.geometric_bounds().as_slice()));
        }
    }

    fn render_rectangle(&self, rect: &Rectangle, parent_transform: &Array2<f64>, page_index: &Option<PdfPageIndex>, layer_index: &Option<PdfLayerIndex>) 
        -> Result<(), String> 
    {
        let item_transform = item_transform_matrix_from_opt_vec!(rect.item_transform());
        
        let points = rect.properties().into_iter()
            .filter_map(|point| point.path_geometry().as_ref())
            .map(|path_geom| path_geom.geometry_path_type().path_point_arrays())
            .flat_map(|path_point_arrays| 
                path_point_arrays.into_iter()
                .flat_map(|path_point_array| 
                    path_point_array.path_point_array().into_iter()
                    .filter_map(|path_point_type| path_point_type.anchor().as_ref())
                )
            )
            .map(|p| item_transform.dot(parent_transform).dot(&arr2(&[[p[0]], [p[1]], [1_f64]])))
            .map(|a| (Point{x: Pt(a[[0,0]]), y: Pt(a[[1,0]])}, false) )
            .collect();

        // Is the shape stroked? Is the shape closed? Is the shape filled?
        let line = Line {
            points: points,
            is_closed: true,
            has_fill: false,
            has_stroke: true,
            is_clipping_path: false,
        };

        // let fill_color = printpdf::Color::Rgb(rect.fill_color());
        let fill_color = printpdf::Color::Rgb(Rgb::new(0.7, 0.2, 0.3, None));
        let line_color = printpdf::Color::Rgb(Rgb::new(0.3, 0.8, 0.7, None));
        
        let layer = match (page_index, layer_index) {
            (&Some(page_id), &Some(layer_id)) => {
                self.pdf_doc.get_page(page_id).get_layer(layer_id)
            },
            (&Some(_), &None) => return Err("No layer index provided".to_string()),
            (&None, &Some(_)) => return Err("No page index provided".to_string()),
            (&None, &None) => return Err("No page and layer index provided".to_string()),
        };

        layer.set_fill_color(fill_color);
        layer.set_outline_color(line_color);
        layer.set_outline_thickness(10.0);

        // Draw first line
        layer.add_shape(line);

        Ok(())
    }

    fn render_oval(&self, oval: &Oval, parent_transform: &Array2<f64>, page_index: &Option<PdfPageIndex>, layer_index: &Option<PdfLayerIndex>) 
        -> Result<(), String> 
    {
        let item_transform = item_transform_matrix_from_opt_vec!(oval.item_transform());
        
        let points = oval.properties().into_iter()
            .filter_map(|point| point.path_geometry().as_ref())
            .map(|path_geom| path_geom.geometry_path_type().path_point_arrays())
            .flat_map(|path_point_arrays| 
                path_point_arrays.into_iter()
                .flat_map(|path_point_array| 
                    path_point_array.path_point_array().into_iter()
                    .filter_map(|path_point_type| path_point_type.anchor().as_ref())
                )
            )
            .map(|p| item_transform.dot(parent_transform).dot(&arr2(&[[p[0]], [p[1]], [1_f64]])))
            .map(|a| (Point{x: Pt(a[[0,0]]), y: Pt(a[[1,0]])}, true) )
            .collect();

        // Is the shape stroked? Is the shape closed? Is the shape filled?
        let line = Line {
            points: points,
            is_closed: true,
            has_fill: false,
            has_stroke: true,
            is_clipping_path: false,
        };

        // let fill_color = printpdf::Color::Rgb(rect.fill_color());
        let fill_color = printpdf::Color::Rgb(Rgb::new(0.7, 0.2, 0.3, None));
        let line_color = printpdf::Color::Rgb(Rgb::new(0.3, 0.8, 0.7, None));
        
        let layer = match (page_index, layer_index) {
            (&Some(page_id), &Some(layer_id)) => {
                self.pdf_doc.get_page(page_id).get_layer(layer_id)
            },
            (&Some(_), &None) => return Err("No layer index provided".to_string()),
            (&None, &Some(_)) => return Err("No page index provided".to_string()),
            (&None, &None) => return Err("No page and layer index provided".to_string()),
        };

        layer.set_fill_color(fill_color);
        layer.set_outline_color(line_color);
        layer.set_outline_thickness(10.0);

        // Draw first line
        layer.add_shape(line);

        Ok(())
    }

    fn render_polygon(&self, polygon: &Polygon, parent_transform: &Array2<f64>, page_index: &Option<PdfPageIndex>, layer_index: &Option<PdfLayerIndex>) 
        -> Result<(), String> 
    {
        let item_transform = item_transform_matrix_from_opt_vec!(polygon.item_transform());
        
        let points = polygon.properties().into_iter()
            .filter_map(|point| point.path_geometry().as_ref())
            .map(|path_geom| path_geom.geometry_path_type().path_point_arrays())
            .flat_map(|path_point_arrays| 
                path_point_arrays.into_iter()
                .flat_map(|path_point_array| 
                    path_point_array.path_point_array().into_iter()
                    .filter_map(|path_point_type| path_point_type.anchor().as_ref())
                )
            )
            .map(|p| item_transform.dot(parent_transform).dot(&arr2(&[[p[0]], [p[1]], [1_f64]])))
            .map(|a| (Point{x: Pt(a[[0,0]]), y: Pt(a[[1,0]])}, false) )
            .collect();

        // Is the shape stroked? Is the shape closed? Is the shape filled?
        let line = Line {
            points: points,
            is_closed: true,
            has_fill: false,
            has_stroke: true,
            is_clipping_path: false,
        };

        // let fill_color = printpdf::Color::Rgb(rect.fill_color());
        let fill_color = printpdf::Color::Rgb(Rgb::new(0.7, 0.2, 0.3, None));
        let line_color = printpdf::Color::Rgb(Rgb::new(0.3, 0.8, 0.7, None));
        
        let layer = match (page_index, layer_index) {
            (&Some(page_id), &Some(layer_id)) => {
                self.pdf_doc.get_page(page_id).get_layer(layer_id)
            },
            (&Some(_), &None) => return Err("No layer index provided".to_string()),
            (&None, &Some(_)) => return Err("No page index provided".to_string()),
            (&None, &None) => return Err("No page and layer index provided".to_string()),
        };

        layer.set_fill_color(fill_color);
        layer.set_outline_color(line_color);
        layer.set_outline_thickness(3.0);

        // Draw first line
        layer.add_shape(line);

        Ok(())
    }

    fn render_textframe(&self, polygon: &TextFrame, parent_transform: &Array2<f64>, page_index: &Option<PdfPageIndex>, layer_index: &Option<PdfLayerIndex>) 
        -> Result<(), String> 
    {
        let item_transform = item_transform_matrix_from_opt_vec!(polygon.item_transform());

        // println!("Parent tranform {:#?}", parent_transform);
        // println!("Tranform {:#?}", item_transform);
        // println!("Combined\n {:#?}", item_transform.dot(parent_transform));
        
        let points = polygon.properties().into_iter()
            .filter_map(|point| point.path_geometry().as_ref())
            .map(|path_geom| path_geom.geometry_path_type().path_point_arrays())
            .flat_map(|path_point_arrays| 
                path_point_arrays.into_iter()
                .flat_map(|path_point_array| 
                    path_point_array.path_point_array().into_iter()
                    .filter_map(|path_point_type| path_point_type.anchor().as_ref())
                )
            )
            .map(|p| item_transform.dot(parent_transform).dot(&arr2(&[[p[0]], [p[1]], [1_f64]])))
            .map(|a| (Point{x: Pt(a[[0,0]]), y: Pt(a[[1,0]])}, false) )
            .collect();

        // Is the shape stroked? Is the shape closed? Is the shape filled?
        let line = Line {
            points: points,
            is_closed: true,
            has_fill: false,
            has_stroke: true,
            is_clipping_path: false,
        };

        // let fill_color = printpdf::Color::Rgb(rect.fill_color());
        let fill_color = printpdf::Color::Rgb(Rgb::new(0.7, 0.2, 0.3, None));
        let line_color = printpdf::Color::Rgb(Rgb::new(0.3, 0.8, 0.7, None));
        
        let layer = match (page_index, layer_index) {
            (&Some(page_id), &Some(layer_id)) => {
                self.pdf_doc.get_page(page_id).get_layer(layer_id)
            },
            (&Some(_), &None) => return Err("No layer index provided".to_string()),
            (&None, &Some(_)) => return Err("No page index provided".to_string()),
            (&None, &None) => return Err("No page and layer index provided".to_string()),
        };

        layer.set_fill_color(fill_color);
        layer.set_outline_color(line_color);
        layer.set_outline_thickness(3.0);

        // Draw first line
        layer.add_shape(line);

        Ok(())
    }

    pub fn save_pdf(self, path: &str) -> Result<(), Error> {
        
        // println!("{:#?}", self.idml_package.master_spreads());
                
        self.pdf_doc.save(&mut BufWriter::new(File::create(path).unwrap()))?;
        
        Ok(())
    }
}
