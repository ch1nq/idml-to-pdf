use super::formats::*;
use derive_getters::Getters;
use serde::Deserialize;
use std::path::Path;

#[derive(Default, Deserialize, Debug, PartialEq, Getters)]
#[serde(rename = "idPkg:Graphic")]
#[serde(rename_all = "PascalCase")]
pub struct IdPkgGraphic {
    #[serde(rename = "DOMVersion")]
    dom_version: Option<f32>,
    #[serde(rename = "Color")]
    colors: Vec<Color>,
    #[serde(rename = "Swatch")]
    swatches: Vec<Swatch>,
}

#[derive(Default, Deserialize, Debug, PartialEq, Getters)]
#[serde(rename_all = "PascalCase")]
pub struct Color {
    #[serde(rename = "Self")]
    id: String,
    name: String,
    #[serde(default, deserialize_with = "deserialize_space_seperated_opt_vec")]
    alternate_color_value: Option<Vec<f64>>,
    alternate_space: Option<ColorSpace>,
    base_color: Option<String>,
    color_override: Option<ColorOverride>,
    #[serde(default, deserialize_with = "deserialize_space_seperated_opt_vec")]
    color_value: Option<Vec<f64>>,
    model: Option<ColorModel>,
    space: Option<ColorSpace>,
    spot_ink_alias_spot_color_reference: Option<String>,
}

#[derive(Default, Deserialize, Debug, PartialEq, Getters)]
#[serde(rename_all = "PascalCase")]
pub struct Swatch {
    #[serde(rename = "Self")]
    id: String,
    name: String,
    color_editable: Option<bool>,
    color_removeable: Option<bool>,
    visible: Option<bool>,
    swatch_creator_id: Option<i32>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub enum ColorSpace {
    RGB,
    CMYK,
    LAB,
    MixedInk,
    NoAlternateColor,
}

#[derive(Deserialize, Debug, PartialEq)]
pub enum ColorOverride {
    Normal,
    Specialpaper,
    Specialblack,
    Specialregistration,
    Hiddenreserved,
    Mixedinkparent,
}

#[derive(Deserialize, Debug, PartialEq)]
pub enum ColorModel {
    Spot,
    Process,
    Registration,
}

pub fn parse_graphic_from_path(path: &Path) -> Result<IdPkgGraphic, quick_xml::DeError> {
    let xml = std::fs::read_to_string(path).unwrap();
    quick_xml::de::from_str(xml.as_str())
}
