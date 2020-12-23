use serde::Deserialize;
use derive_getters::Getters;
use super::{Style, StyleGroup};

#[derive(Default,Deserialize,Debug,PartialEq,Getters)]
#[serde(rename_all="PascalCase")]
pub struct RootObjectStyleGroup {
    #[serde(rename="Self")]
    id: Option<String>,
    #[serde(rename="ObjectStyle")]
    object_styles: Option<Vec<ObjectStyle>>,
}

#[derive(Default,Deserialize,Debug,PartialEq,Getters)]
#[serde(rename_all="PascalCase")]
pub struct ObjectStyle {
    #[serde(rename="Self")]
    id: Option<String>, 
    name: Option<String>,
    fill_color: Option<String>,
    stroke_weight: Option<f64>,
    stroke_color: Option<String>,
}

impl Style for ObjectStyle {
    fn get_id(&self) -> &Option<String> { &self.id() }
}

impl StyleGroup<ObjectStyle> for RootObjectStyleGroup {
    fn get_styles(&self) -> &Option<Vec<ObjectStyle>> { &self.object_styles() }
}
