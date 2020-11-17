use std::fs::File;
use std::path::Path;

#[derive(Default, Deserialize,Debug)]
#[serde(rename="idPkg:Story")]
#[serde(rename_all="PascalCase")]
pub struct SpreadWrapper {
    #[serde(rename="DOMVersion")]
    dom_version: Option<f32>,
    spread: Option<Spread>,
}

#[derive(Default, Deserialize,Debug)]
#[serde(rename_all="PascalCase")]
pub struct Spread {
    #[serde(rename="Self")]
    id: String,
    user_text: Option<bool>,
    spread_title: Option<String>,
    #[serde(rename="ParagraphStyleRange")]
    paragraph_style_ranges: Option<Vec<ParagraphStyleRange>>
}

#[derive(Default,Deserialize,Debug)]
#[serde(rename_all="PascalCase")]
pub struct ParagraphStyleRange {
    applied_paragraph_style: Option<String>,
    justification: Option<String>,
    #[serde(rename="CharacterStyleRange")]
    character_style_ranges: Option<Vec<CharacterStyleRange>>
}

#[derive(Default,Deserialize,Debug)]
#[serde(rename_all="PascalCase")]
pub struct CharacterStyleRange {
    applied_character_style: Option<String>,
    fill_color: Option<String>,
    font_style: Option<String>, 
    point_size: Option<String>,
    properties: Option<Properties>,
    #[serde(rename="Content")]
    contents: Option<Vec<Content>>
}

#[derive(Default,Deserialize,Debug)]
#[serde(rename_all="PascalCase")]
struct Properties {
    applied_font: Option<String> 
}

#[derive(Default,Deserialize,Debug)]
#[serde(rename_all="PascalCase")]
pub struct Content {
    #[serde(rename="$value")]
    text: String,
}

pub fn parse_spread_from_path(path: &Path) -> Result<SpreadWrapper, serde_xml_rs::Error> {
    let xml = std::fs::read_to_string(path).unwrap();
    serde_xml_rs::from_str(xml.as_str())
}

impl SpreadWrapper {
    pub fn get_spread(self) -> Option<Spread> {
        self.spread
    }
}

impl Spread {
    pub fn get_id(self) -> &str {
        &self.id
    }
}
