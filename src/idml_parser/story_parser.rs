use std::path::Path;
use quick_xml::de::{from_str, DeError};

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
    // #[serde(deserialize_with = "deserialize_content", flatten)]
    // #[serde(rename="Content", alias="Br")]
    // #[serde(rename="Content")]
    contents: Option<Vec<Content>>,
}

#[derive(Default,Deserialize,Debug)]
#[serde(rename_all="PascalCase")]
struct Properties {
    applied_font: Option<String> 
}

// #[derive(Deserialize,Debug)]
// enum ContentOrLineBreak {
//     Content(String),
//     Br(String)
// }

#[derive(Default,Deserialize,Debug)]
#[serde(rename_all="PascalCase")]
pub struct Content {
    #[serde(rename="$value")]
    text: Option<String>,
    // br: Option<String>
}

// fn deserialize_content<'d, D: Deserializer<'d>>(d: D) -> Result<Option<String>, D::Error> {
//     let Content { text, br } = Content::deserialize(d)?;
//     Ok(text.or(br))
// }

// #[derive(Default,Deserialize,Debug)]
// pub struct Br {
//     text: String,
// }

pub fn parse_story_from_path(path: &Path) -> Result<StoryWrapper, DeError> {
    let xml = std::fs::read_to_string(path).unwrap();
    
    // let xml = r##"
    // <idPkg:Story xmlns:idPkg="http://ns.adobe.com/AdobeInDesign/idml/1.0/packaging" DOMVersion="15.0">
	// <Story Self="u373" AppliedTOCStyle="n" UserText="true" IsEndnoteStory="false" TrackChanges="false" StoryTitle="$ID/" AppliedNamedGrid="n">
	// 	<ParagraphStyleRange AppliedParagraphStyle="ParagraphStyle/$ID/NormalParagraphStyle" Justification="CenterAlign">
    //         <CharacterStyleRange>
    //             <Properties>
    //                 <SomeProperty>value</SomeProperty>
    //             </Properties>
    //             <Content>First line</Content>
    //             <Br />
    //             <Content>Second line</Content>
    //             <Br />
    //             <Content>Third line</Content>
    //         </CharacterStyleRange>
    //     </ParagraphStyleRange>
    // </Story>
    // </idPkg:Story>
    // "##.to_string();
    
    // serde_xml_rs::from_str(xml.as_str())
    from_str(xml.as_str())
}

impl StoryWrapper {
    pub fn get_story(self) -> Option<Story> {
        self.story
    }
}

impl Story {
    // pub fn get_id(self) -> String {
    //     self.id
    // }
}
