use crate::idml_parser::styles::commom_text_properties::*;
use derive_getters::Getters;
use quick_xml::de::{from_str, DeError};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

#[derive(Default, Deserialize, Debug, Getters)]
#[serde(rename = "idPkg:Story")]
#[serde(rename_all = "PascalCase")]
pub struct StoryWrapper {
    #[serde(rename = "DOMVersion")]
    dom_version: Option<f32>,
    story: Option<Story>,
}

#[derive(Default, Deserialize, Debug, Getters)]
#[serde(rename_all = "PascalCase")]
pub struct Story {
    #[serde(rename = "Self")]
    id: String,
    user_text: Option<bool>,
    story_title: Option<String>,
    #[serde(rename = "ParagraphStyleRange")]
    paragraph_style_ranges: Option<Vec<ParagraphStyleRange>>,
}

#[derive(Default, Deserialize, Debug, PartialEq, Getters)]
#[serde(rename_all = "PascalCase")]
pub struct ParagraphStyleRange {
    #[serde(rename = "CharacterStyleRange")]
    character_style_ranges: Option<Vec<CharacterStyleRange>>,
    properties: Option<Properties>,

    #[serde(flatten)]
    // ctp_fields: HashMap<CTPKey, CTPValue>,
    ctp_fields: CTPMap,
}

impl_common_text_properties!(ParagraphStyleRange);

#[derive(Default, Deserialize, Debug, PartialEq, Getters)]
#[serde(rename_all = "PascalCase")]
pub struct CharacterStyleRange {
    #[serde(rename = "$value")]
    contents: Option<Vec<StoryContent>>,
    properties: Option<Properties>,

    #[serde(flatten)]
    // ctp_fields: HashMap<CTPKey, CTPValue>,
    ctp_fields: CTPMap,
}

impl_common_text_properties!(CharacterStyleRange);

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum StoryContent {
    Properties(Properties),
    Content(String),
    Br,
    #[serde(other)]
    NotImplementedYet,
}

#[derive(Default, Deserialize, Debug, Getters)]
#[serde(rename_all = "PascalCase")]
pub struct Content {
    #[serde(rename = "$value")]
    text: Option<String>,
}

pub fn parse_story_from_path(path: &Path) -> Result<StoryWrapper, DeError> {
    let xml = std::fs::read_to_string(path).unwrap();
    from_str(xml.as_str())
}

impl StoryWrapper {
    pub fn get_story(self) -> Option<Story> {
        self.story
    }
}
