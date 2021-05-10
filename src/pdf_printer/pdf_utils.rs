use crate::idml_parser::spread_parser::TextFrame;
use crate::pdf_printer::color_manager::Color;
use crate::pdf_printer::transforms::{self, *};
use libharu_sys::*;

pub fn set_fill_color(page: HPDF_Page, fill_color: Color) {
    unsafe {
        match fill_color {
            Color::Cmyk(color) => {
                HPDF_Page_SetCMYKFill(page, *color.c(), *color.m(), *color.y(), *color.k());
            }
            Color::Rgb(color) => {
                HPDF_Page_SetRGBFill(page, *color.r(), *color.g(), *color.b());
            }
            _ => {}
        }
    }
}

pub fn set_stroke_color(page: HPDF_Page, stroke_color: Color) {
    unsafe {
        match stroke_color {
            Color::Cmyk(color) => {
                HPDF_Page_SetCMYKStroke(page, *color.c(), *color.m(), *color.y(), *color.k());
            }
            Color::Rgb(color) => {
                HPDF_Page_SetRGBStroke(page, *color.r(), *color.g(), *color.b());
            }
            _ => {}
        }
    }
}

#[derive(Debug, Clone)]
pub struct BoundingBox {
    pub left: f64,
    pub right: f64,
    pub top: f64,
    pub bottom: f64,
}

pub fn boundingbox(textframe: &TextFrame, parent_transform: &Transform) -> BoundingBox {
    let item_transform = transforms::from_vec(textframe.item_transform());

    let points: Vec<(f64, f64)> = textframe
        .properties()
        .into_iter()
        .filter_map(|point| point.path_geometry().as_ref())
        .map(|path_geom| path_geom.geometry_path_type().path_point_arrays())
        .flat_map(|path_point_arrays| {
            path_point_arrays
                .into_iter()
                .flat_map(|path_point_array| {
                    path_point_array
                        .path_point_array()
                        .into_iter()
                        .filter_map(|path_point_type| path_point_type.anchor().as_ref())
                        .map(|point| {
                            item_transform
                                .combine_with(&parent_transform)
                                .apply_to_point(&point[0], &point[1])
                        })
                        .map(|point| (point[0], point[1]))
                })
                .into_iter()
        })
        .collect();

    // Left, right, top and bottom coordinates
    let &(left, _) = points
        .iter()
        .min_by(|(x1, _), (x2, _)| x1.partial_cmp(&x2).unwrap())
        .unwrap();
    let &(right, _) = points
        .iter()
        .max_by(|(x1, _), (x2, _)| x1.partial_cmp(&x2).unwrap())
        .unwrap();
    let &(_, top) = points
        .iter()
        .max_by(|(_, y1), (_, y2)| y1.partial_cmp(&y2).unwrap())
        .unwrap();
    let &(_, bottom) = points
        .iter()
        .min_by(|(_, y1), (_, y2)| y1.partial_cmp(&y2).unwrap())
        .unwrap();

    BoundingBox {
        left,
        right,
        top,
        bottom,
    }
}
