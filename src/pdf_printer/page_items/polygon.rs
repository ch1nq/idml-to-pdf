use crate::idml_parser::spread_parser::*;
use crate::idml_parser::IDMLResources;
// use crate::pdf_printer::pdf_utils;
use crate::pdf_printer::color_manager::{Color};
use crate::pdf_printer::transforms::{self, Transform};
// use printpdf::indices::{PdfLayerIndex, PdfPageIndex};
// use printpdf::{Line, HPDF_Doc, Point, Pt};
use libharu_sys::*;

pub trait IsPolygon {
    fn get_properties(&self) -> &Option<Properties>;
    fn get_item_transform(&self) -> &Option<Vec<f64>>;
    fn get_fill_color(&self) -> &Option<String>;
    fn get_stroke_color(&self) -> &Option<String>;
    fn get_stroke_weight(&self) -> &Option<f64>;
    fn get_object_style(&self) -> &Option<String>;
}

impl IsPolygon for Polygon {
    fn get_properties(&self) -> &Option<Properties> {
        &self.properties()
    }
    fn get_item_transform(&self) -> &Option<Vec<f64>> {
        &self.item_transform()
    }
    fn get_fill_color(&self) -> &Option<String> {
        &self.fill_color()
    }
    fn get_stroke_color(&self) -> &Option<String> {
        &self.stroke_color()
    }
    fn get_stroke_weight(&self) -> &Option<f64> {
        &self.stroke_weight()
    }
    fn get_object_style(&self) -> &Option<String> {
        &self.applied_object_style()
    }
}

impl IsPolygon for Rectangle {
    fn get_properties(&self) -> &Option<Properties> {
        &self.properties()
    }
    fn get_item_transform(&self) -> &Option<Vec<f64>> {
        &self.item_transform()
    }
    fn get_fill_color(&self) -> &Option<String> {
        &self.fill_color()
    }
    fn get_stroke_color(&self) -> &Option<String> {
        &self.stroke_color()
    }
    fn get_stroke_weight(&self) -> &Option<f64> {
        &self.stroke_weight()
    }
    fn get_object_style(&self) -> &Option<String> {
        &self.applied_object_style()
    }
}

impl IsPolygon for Oval {
    fn get_properties(&self) -> &Option<Properties> {
        &self.properties()
    }
    fn get_item_transform(&self) -> &Option<Vec<f64>> {
        &self.item_transform()
    }
    fn get_fill_color(&self) -> &Option<String> {
        &self.fill_color()
    }
    fn get_stroke_color(&self) -> &Option<String> {
        &self.stroke_color()
    }
    fn get_stroke_weight(&self) -> &Option<f64> {
        &self.stroke_weight()
    }
    fn get_object_style(&self) -> &Option<String> {
        &self.applied_object_style()
    }
}

impl IsPolygon for TextFrame {
    fn get_properties(&self) -> &Option<Properties> {
        &self.properties()
    }
    fn get_item_transform(&self) -> &Option<Vec<f64>> {
        &self.item_transform()
    }
    fn get_fill_color(&self) -> &Option<String> {
        &self.fill_color()
    }
    fn get_stroke_color(&self) -> &Option<String> {
        &self.stroke_color()
    }
    fn get_stroke_weight(&self) -> &Option<f64> {
        &self.stroke_weight()
    }
    fn get_object_style(&self) -> &Option<String> {
        &self.applied_object_style()
    }
}

pub trait RenderPolygon {
    fn render(
        &self,
        parent_transform: &Transform,
        idml_resources: &IDMLResources,
        current_page: HPDF_Page,
    ) -> Result<(), String>;
}

impl<T: IsPolygon> RenderPolygon for T {
    fn render(
        &self,
        parent_transform: &Transform,
        idml_resources: &IDMLResources,
        current_page: HPDF_Page,
    ) -> Result<(), String> {
    

        let item_transform = transforms::from_vec(self.get_item_transform());

        // Parse the points and apply the relevant transformations
        let mut points: Vec<HPDF_REAL> = self
            .get_properties()
            .into_iter()
            .filter_map(|properties| properties.path_geometry().as_ref())
            .map(|path_geom| path_geom.geometry_path_type().path_point_arrays())
            .flat_map(|path_point_arrays| {
                path_point_arrays.into_iter().flat_map(|path_point_array| {
                    path_point_array
                        .path_point_array()
                        .into_iter()
                        .map(|path_point_type| {
                            [
                                // Get anchor point and its two handles for the beizer curve
                                path_point_type.anchor().as_ref(),
                                path_point_type.left_direction().as_ref(),
                                path_point_type.right_direction().as_ref(),
                            ]
                        })
                        .filter(|[a, l, r]| a.is_some() && l.is_some() && r.is_some())
                        .map(|[a, l, r]| [a.unwrap(), l.unwrap(), r.unwrap()])
                        .map(|[a, l, r]| {
                            [
                                // Apply item and parent transformation matrices
                                item_transform
                                    .combine_with(&parent_transform)
                                    .apply_to_point(&a[0], &a[1]),
                                item_transform
                                    .combine_with(&parent_transform)
                                    .apply_to_point(&l[0], &l[1]),
                                item_transform
                                    .combine_with(&parent_transform)
                                    .apply_to_point(&r[0], &r[1]),
                            ]
                        })
                        .flat_map(|[a, l, r]| {
                            vec![
                                l[0] as f32, l[1] as f32, // Left handle
                                a[0] as f32, a[1] as f32, // Anchor
                                r[0] as f32, r[1] as f32, // Right handle
                            ]
                            .into_iter()
                        })
                })
            })
            .collect();

        // Initialize fill and stroke color to None
        let mut fill_color = idml_resources.color_from_id(&"Swatch/None".to_string());
        let mut stroke_color = idml_resources.color_from_id(&"Swatch/None".to_string());
        let mut stroke_weight = None;

        // If a graphic style is applied, then update fill and stroke color from that
        if let Some(style_id) = self.get_object_style() {
            if let Some(style) = idml_resources.styles().object_style_from_id(style_id) {
                if let Some(id) = style.fill_color() {
                    fill_color = idml_resources.color_from_id(id);
                }
                if let Some(id) = style.stroke_color() {
                    stroke_color = idml_resources.color_from_id(id);
                }
                stroke_weight = style.stroke_weight().to_owned();
            }
        }

        // Override fill color if one is available on the polygon
        if let Some(id) = self.get_fill_color() {
            fill_color = idml_resources.color_from_id(id)
        }
        
        // Override stroke color if one is available on the polygon
        if let Some(id) = self.get_stroke_color() {
            stroke_color = idml_resources.color_from_id(id)
        }
        
        // Override stroke weight if one is available on the polygon
        if let Some(weight) = self.get_stroke_weight() {
            stroke_weight = Some(weight.to_owned());
        }

        let closed_path = match self.get_properties() {
            Some(properties) => {
                if let Some(path_geom) = properties.path_geometry() {
                    !*path_geom.geometry_path_type().path_open()
                } else {
                    false
                }
            }
            _ => false
        };
        
        unsafe {
            // Save the current graphic state 
            HPDF_Page_GSave(current_page);  
            
            // Set line thickness
            if let Some(weight) = stroke_weight {
                HPDF_Page_SetLineWidth(current_page, weight.to_owned() as f32);
            };
            
            // Set fill color of shape
            match fill_color {
                Ok(Color::Cmyk(color)) => {
                    HPDF_Page_SetCMYKFill(current_page, *color.c(), *color.m(), *color.y(), *color.k());
                },
                Ok(Color::Rgb(color)) => {
                    HPDF_Page_SetRGBFill(current_page, *color.r(), *color.g(), *color.b());
                },
                _ => {}
            }
            
            // Set stroke color of shape
            match stroke_color {
                Ok(Color::Cmyk(color)) => {
                    HPDF_Page_SetCMYKStroke(current_page, *color.c(), *color.m(), *color.y(), *color.k());
                },
                Ok(Color::Rgb(color)) => {
                    HPDF_Page_SetRGBStroke(current_page, *color.r(), *color.g(), *color.b());
                },
                _ => {}
            }
            
            // The PDF library wants the points in a slightly different order
            // We just need to rotate the vec twice
            points.rotate_right(2);
            // points.append(&mut vec!(points[0], points[1]));

            println!("");

            // Start drawing from first anchor point
            if points.len() >= 2 {
                let l = points.len();
                HPDF_Page_MoveTo(current_page, points[l-2], points[l-1]);
            }
            
            // Loop over anchor points and bezier handles 
            for slice in points.chunks(6) {
                println!("{:?}", slice);
                if let &[rx, ry, lx, ly, ax, ay] = slice {
                    HPDF_Page_CurveTo(current_page, rx, ry, lx, ly, ax, ay);
                }
            }

            match (closed_path, fill_color.is_ok(), stroke_color.is_ok()) {
                (_,     true,  false) => HPDF_Page_Fill(current_page),
                (false, false, true) =>  HPDF_Page_Stroke(current_page),
                (false, true,  true) =>  HPDF_Page_FillStroke(current_page),
                (true,  false, true) =>  HPDF_Page_ClosePathStroke(current_page),
                (true,  true,  true) =>  HPDF_Page_ClosePathFillStroke(current_page),
                _ => HPDF_Page_EndPath(current_page),
            };
            
            // Restore the previous graphic state 
            HPDF_Page_GRestore(current_page);  
        }
        Ok(())
    }
}
