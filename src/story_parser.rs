use std::fs::File;
use std::path::Path;

#[derive(Default, Deserialize,Debug)]
#[serde(rename="idPkg:Story")]
#[serde(rename_all="PascalCase")]
pub struct StoryWrapper {
    #[serde(rename="DOMVersion")]
    dom_version: Option<f32>,
    story: Option<Story>,
}

#[derive(Default, Deserialize,Debug)]
#[serde(rename_all="PascalCase")]
pub struct Story {
    #[serde(rename="Self")]
    id: String,
    user_text: Option<bool>,
    story_title: Option<String>,
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

pub fn parse_story_from_path(path: &Path) -> StoryWrapper {
    let xml = std::fs::read_to_string(path).unwrap();
    let story = serde_xml_rs::from_str(xml.as_str()).unwrap();
    
    story
}
