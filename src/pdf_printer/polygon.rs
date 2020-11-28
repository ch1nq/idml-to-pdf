use printpdf::{PdfDocumentReference, Point, Pt, Line};
use printpdf::indices::{PdfLayerIndex, PdfPageIndex};
use crate::idml_parser::spread_parser::*;
use super::transforms::{self, Transform};
use super::color_manager;
use crate::idml_parser::package_parser::IDMLResources;


pub trait IsPolygon {
    fn get_properties(&self) -> &Option<Properties>;
    fn get_item_transform(&self) -> &Option<Vec<f64>>;
    fn get_fill_color(&self) -> &Option<String>;
    fn get_stroke_color(&self) -> &Option<String>;
    fn get_stroke_weight(&self) -> &Option<f64>;
}

impl IsPolygon for Polygon {
    fn get_properties(&self) -> &Option<Properties> { &self.properties() }
    fn get_item_transform(&self) -> &Option<Vec<f64>> { &self.item_transform() }
    fn get_fill_color(&self) -> &Option<String> { &self.fill_color() }
    fn get_stroke_color(&self) -> &Option<String> { &self.stroke_color() }
    fn get_stroke_weight(&self) -> &Option<f64> { &self.stroke_weight() }
}

impl IsPolygon for Rectangle {
    fn get_properties(&self) -> &Option<Properties> { &self.properties() }
    fn get_item_transform(&self) -> &Option<Vec<f64>> { &self.item_transform() }
    fn get_fill_color(&self) -> &Option<String> { &self.fill_color() }
    fn get_stroke_color(&self) -> &Option<String> { &self.stroke_color() }
    fn get_stroke_weight(&self) -> &Option<f64> { &self.stroke_weight() }
}

impl IsPolygon for Oval {
    fn get_properties(&self) -> &Option<Properties> { &self.properties() }
    fn get_item_transform(&self) -> &Option<Vec<f64>> { &self.item_transform() }
    fn get_fill_color(&self) -> &Option<String> { &self.fill_color() }
    fn get_stroke_color(&self) -> &Option<String> { &self.stroke_color() }
    fn get_stroke_weight(&self) -> &Option<f64> { &self.stroke_weight() }
}

impl IsPolygon for TextFrame {
    fn get_properties(&self) -> &Option<Properties> { &self.properties() }
    fn get_item_transform(&self) -> &Option<Vec<f64>> { &self.item_transform() }
    fn get_fill_color(&self) -> &Option<String> { &self.fill_color() }
    fn get_stroke_color(&self) -> &Option<String> { &self.stroke_color() }
    fn get_stroke_weight(&self) -> &Option<f64> { &self.stroke_weight() }
}

pub trait RenderPolygon {
    fn render(&self, 
        parent_transform: &Transform, 
        pdf_doc: &PdfDocumentReference, 
        idml_resources: &IDMLResources,
        page_index: &Option<PdfPageIndex>, 
        layer_index: &Option<PdfLayerIndex>
    ) -> Result<(), String>;
}

impl<T: IsPolygon> RenderPolygon for T {
    fn render(&self, 
        parent_transform: &Transform, 
        pdf_doc: &PdfDocumentReference, 
        idml_resources: &IDMLResources, 
        page_index: &Option<PdfPageIndex>, 
        layer_index: &Option<PdfLayerIndex>
    ) -> Result<(), String>
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
        // Lookup fill color 
        let fill_color = match self.get_fill_color() {
            Some(id) => idml_resources.color_from_id(id),
            None => idml_resources.color_from_id(&"Swatch/None".to_string())
        };
        
        println!("{:?}", self.get_stroke_color());
        
        // Lookup stroke color 
        let stroke_color = match self.get_stroke_color() {
            Some(id) => idml_resources.color_from_id(id),
            None => idml_resources.color_from_id(&"Swatch/None".to_string())
        };

        // Is the shape stroked? Is the shape closed? Is the shape filled?
        let line = Line {
            points: points,
            is_closed: true,
            has_fill: fill_color.is_some(),
            has_stroke: stroke_color.is_some(),
            is_clipping_path: false,
        };
        
        let layer = match (page_index, layer_index) {
            (&Some(page_id), &Some(layer_id)) => {
                pdf_doc.get_page(page_id).get_layer(layer_id)
            },
            (&Some(_), &None) => return Err("No layer index provided".to_string()),
            (&None, &Some(_)) => return Err("No page index provided".to_string()),
            (&None, &None) => return Err("No page and layer index provided".to_string()),
        };

        // Set fill color
        if let Some(color) = fill_color {
            layer.set_fill_color(color);
        };
        
        // Set stroke color
        if let Some(color) = stroke_color {
            layer.set_outline_color(color);
        };
        
        // Set stroke thickness
        if let Some(thickness) = &self.get_stroke_weight() {
            layer.set_outline_thickness(thickness.to_owned());
        };

        // Draw first line
        layer.add_shape(line);

        Ok(())
    }
}

