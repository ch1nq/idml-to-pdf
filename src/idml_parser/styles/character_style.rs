use serde::Deserialize;
use derive_getters::Getters;
use super::{Style, StyleGroup};

#[derive(Default,Deserialize,Debug,PartialEq,Getters)]
#[serde(rename_all="PascalCase")]
pub struct RootCharacterStyleGroup {
    #[serde(rename="Self")]
    id: Option<String>,
    #[serde(rename="ObjectStyle")]
    object_styles: Option<Vec<CharacterStyle>>,
}

#[derive(Default,Deserialize,Debug,PartialEq,Getters)]
#[serde(rename_all="PascalCase")]
pub struct CharacterStyle {
    #[serde(rename="Self")]
    id: Option<String>, 
    name: Option<String>,
    fill_color: Option<String>,
    stroke_weight: Option<f64>,
    stroke_color: Option<String>,
}

impl Style for CharacterStyle {
    fn get_id(&self) -> &Option<String> { &self.id() }
}

impl StyleGroup<CharacterStyle> for RootCharacterStyleGroup {
    fn get_styles(&self) -> &Option<Vec<CharacterStyle>> { &self.object_styles() }
}
