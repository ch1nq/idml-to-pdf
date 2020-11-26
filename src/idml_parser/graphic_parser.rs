use serde::Deserialize;
use derive_getters::Getters;
use super::formats::*;

#[derive(Default, Deserialize,Debug,PartialEq,Getters)]
#[serde(rename="idPkg:Graphic")]
#[serde(rename_all="PascalCase")]
pub struct IdPkgGraphic {
    #[serde(rename="DOMVersion")]
    dom_version: Option<f32>,
    #[serde(rename="Color")]
    colors: Vec<Color>
}

#[derive(Default, Deserialize,Debug,PartialEq,Getters)]
#[serde(rename_all="PascalCase")]
pub struct Color {
    #[serde(rename="Self")]
    id: Option<String>,
    #[serde(deserialize_with="deserialize_space_seperated_opt_vec")]
    alternate_color_value: Option<Vec<f64>>,
    alternate_space: Option<ColorSpace>,
    base_color: Option<String>,
    color_override: Option<ColorOverride>,
    #[serde(deserialize_with="deserialize_space_seperated_opt_vec")]
    color_value: Option<Vec<f64>>,
    model: Option<ColorModel>,
    space: Option<ColorSpace>,
    spot_ink_alias_spot_color_reference: Option<String>
}

#[derive(Deserialize,Debug,PartialEq)]
pub enum ColorSpace {
    RGB,
    CMYK,
    LAB,
    MixedInk,
    NoAlternateColor
}

#[derive(Deserialize,Debug,PartialEq)]
pub enum ColorOverride {
    Normal, 
    Specialpaper, 
    Specialblack, 
    Specialregistration, 
    Hiddenreserved,
    Mixedinkparent
}

#[derive(Deserialize,Debug,PartialEq)]
pub enum ColorModel {
    Spot,
    Process
}

