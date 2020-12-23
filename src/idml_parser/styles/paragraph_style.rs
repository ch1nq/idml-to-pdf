use serde::Deserialize;
use derive_getters::Getters;
use super::{Style, StyleGroup};

#[derive(Default,Deserialize,Debug,PartialEq,Getters)]
#[serde(rename_all="PascalCase")]
pub struct RootParagraphStyleGroup {
    #[serde(rename="Self")]
    id: Option<String>,
    #[serde(rename="ObjectStyle")]
    object_styles: Option<Vec<ParagraphStyle>>,
}

#[derive(Default,Deserialize,Debug,PartialEq,Getters)]
#[serde(rename_all="PascalCase")]
pub struct ParagraphStyle {
    #[serde(rename="Self")]
    id: Option<String>, 
    name: Option<String>,
    fill_color: Option<String>,
    stroke_weight: Option<f64>,
    stroke_color: Option<String>,
}

impl Style for ParagraphStyle {
    fn get_id(&self) -> &Option<String> { &self.id() }
}

impl StyleGroup<ParagraphStyle> for RootParagraphStyleGroup {
    fn get_styles(&self) -> &Option<Vec<ParagraphStyle>> { &self.object_styles() }
}
