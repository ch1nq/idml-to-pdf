use std::fs::File;
use std::io::BufWriter;
use printpdf::*;
use printpdf::indices::{PdfLayerIndex, PdfPageIndex};
use crate::idml_parser::package_parser::IDMLPackage;
use crate::idml_parser::spread_parser::*;
use super::transforms::{self, Transform};
use super::color_manager;

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
                r.render(page_transform, &self.pdf_doc, page_index, layer_index)
                .expect(format!("Failed to render rectangle '{}'", r.id()).as_str())
            }
            SpreadContent::Polygon(p) => { 
                p.render(page_transform, &self.pdf_doc, page_index, layer_index)
                .expect(format!("Failed to render polygon '{}'", p.id()).as_str())
            }
            SpreadContent::TextFrame(t) => { 
                t.render(page_transform, &self.pdf_doc, page_index, layer_index)
                .expect(format!("Failed to render textframe '{}'", t.id()).as_str())
            }
            SpreadContent::Oval(o) => { 
                o.render(page_transform, &self.pdf_doc, page_index, layer_index)
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

pub trait IsPolygon {
    fn get_properties(&self) -> &Option<Properties>;
    fn get_item_transform(&self) -> &Option<Vec<f64>>;
    fn get_fill_color(&self) -> &Option<String>;
    fn get_stroke_color(&self) -> &Option<String>;
}

impl IsPolygon for Polygon {
    fn get_properties(&self) -> &Option<Properties> { &self.properties() }
    fn get_item_transform(&self) -> &Option<Vec<f64>> { &self.item_transform() }
    fn get_fill_color(&self) -> &Option<String> { &self.fill_color() }
    fn get_stroke_color(&self) -> &Option<String> { &self.stroke_color() }
}

impl IsPolygon for Rectangle {
    fn get_properties(&self) -> &Option<Properties> { &self.properties() }
    fn get_item_transform(&self) -> &Option<Vec<f64>> { &self.item_transform() }
    fn get_fill_color(&self) -> &Option<String> { &self.fill_color() }
    fn get_stroke_color(&self) -> &Option<String> { &self.stroke_color() }
}

impl IsPolygon for Oval {
    fn get_properties(&self) -> &Option<Properties> { &self.properties() }
    fn get_item_transform(&self) -> &Option<Vec<f64>> { &self.item_transform() }
    fn get_fill_color(&self) -> &Option<String> { &self.fill_color() }
    fn get_stroke_color(&self) -> &Option<String> { &self.stroke_color() }
}

impl IsPolygon for TextFrame {
    fn get_properties(&self) -> &Option<Properties> { &self.properties() }
    fn get_item_transform(&self) -> &Option<Vec<f64>> { &self.item_transform() }
    fn get_fill_color(&self) -> &Option<String> { &self.fill_color() }
    fn get_stroke_color(&self) -> &Option<String> { &self.stroke_color() }
}

pub trait RenderPolygon {
    fn render(&self, parent_transform: &Transform, pdf_doc: &PdfDocumentReference, page_index: &Option<PdfPageIndex>, layer_index: &Option<PdfLayerIndex>) 
        -> Result<(), String>;
}

impl<T: IsPolygon> RenderPolygon for T {
    fn render(&self, parent_transform: &Transform, pdf_doc: &PdfDocumentReference, page_index: &Option<PdfPageIndex>, layer_index: &Option<PdfLayerIndex>) 
        -> Result<(), String>
    {
        let item_transform = transforms::from_vec(self.get_item_transform());
        
        // Parse the points and apply the relevant transformations
        let mut points: Vec<(Point, bool)> = self.get_properties().into_iter()
            .filter_map(|point| point.path_geometry().as_ref())
            .map(|path_geom| path_geom.geometry_path_type().path_point_arrays())
            .flat_map(|path_point_arrays| 
                path_point_arrays.into_iter()
                .flat_map(|path_point_array| 
                    path_point_array.path_point_array().into_iter()
                    .map(|path_point_type| 
                        [
                            // Get anchor point and its two handles for the beizer curve
                            path_point_type.anchor().as_ref(), 
                            path_point_type.left_direction().as_ref(),
                            path_point_type.right_direction().as_ref()
                        ] 
                    )
                    .filter(|[a,l,r]| a.is_some() && l.is_some() && r.is_some())
                    .map(|[a,l,r]| [a.unwrap(), l.unwrap(), r.unwrap()] )
                    .map(|[a,l,r]| 
                        [
                            // Apply item and parent transformation matrices
                            item_transform.combine_with(&parent_transform).apply_to_point(&a[0],&a[1]),
                            item_transform.combine_with(&parent_transform).apply_to_point(&l[0],&l[1]),
                            item_transform.combine_with(&parent_transform).apply_to_point(&r[0],&r[1]),
                        ]
                    )
                    .flat_map(|[a,l,r]| 
                        vec![
                            // PDF library wants beizer curves in this order:
                            (Point{x: Pt(l[0]), y: Pt(l[1])}, true),    // Left handle
                            (Point{x: Pt(a[0]), y: Pt(a[1])}, false),   // Anchor
                            (Point{x: Pt(a[0]), y: Pt(a[1])}, true),    // Anchor
                            (Point{x: Pt(r[0]), y: Pt(r[1])}, true),    // Right handle
                        ].into_iter()
                    )
                )
            )
            .collect();
        
        // The PDF library wants the points in a slightly different order
        // We just need to rotate the vec twice 
        points.rotate_right(2);

        // Is the shape stroked? Is the shape closed? Is the shape filled?
        let line = Line {
            points: points,
            is_closed: true,
            has_fill: true,
            has_stroke: true,
            is_clipping_path: false,
        };
        
        let fill_color = match self.get_fill_color() {
            Some(id) => color_manager::color_from_id(id),
            None => color_manager::color_from_id(&"Swatch/None".to_string())
        };

        let stroke_color = match self.get_stroke_color() {
            Some(id) => color_manager::color_from_id(id),
            None => color_manager::color_from_id(&"Swatch/None".to_string())
        };

        let layer = match (page_index, layer_index) {
            (&Some(page_id), &Some(layer_id)) => {
                pdf_doc.get_page(page_id).get_layer(layer_id)
            },
            (&Some(_), &None) => return Err("No layer index provided".to_string()),
            (&None, &Some(_)) => return Err("No page index provided".to_string()),
            (&None, &None) => return Err("No page and layer index provided".to_string()),
        };

        layer.set_fill_color(fill_color);
        layer.set_outline_color(stroke_color);
        layer.set_outline_thickness(2.0);

        // Draw first line
        layer.add_shape(line);

        Ok(())
    }
}

