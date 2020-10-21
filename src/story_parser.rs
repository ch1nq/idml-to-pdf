use std::fs::File;
use std::path::Path;

#[derive(Deserialize,Debug)]
pub struct Story {
    // Self: String,
    // UserText: bool,
    // StoryTitle: String,
    #[serde(rename="ParagraphStyleRange")]
    ParagraphStyleRanges: Vec<ParagraphStyleRange>
}

#[derive(Deserialize,Debug)]
pub struct ParagraphStyleRange {
    AppliedParagraphStyle: String,
    #[serde(rename="CharacterStyleRange")]
    CharacterStyleRanges: Vec<CharacterStyleRange>
}

#[derive(Deserialize,Debug)]
pub struct CharacterStyleRange {
    AppliedCharacterStyle: String,
    // FillColor: String,
    // FontStyle: String, 
    // PointSize: String,
    #[serde(rename="Content")]
    Contents: Vec<Content>
}

#[derive(Deserialize,Debug)]
pub struct Content {
    #[serde(rename="$value")]
    text: String,
}

pub fn parse_story_from_path(path: &Path) -> Story {
    let test_xml = r##"<Story><ParagraphStyleRange AppliedParagraphStyle="ParagraphStyle\kNormalParagraphStyle"> <CharacterStyleRange AppliedCharacterStyle="CharacterStyle\k[No character style]"
    FillColor="Color\cRed" FontStyle="Italic" PointSize="24"> <Content>ABC</Content>
    <br/>
    </CharacterStyleRange>
    </ParagraphStyleRange>
    <ParagraphStyleRange AppliedParagraphStyle="ParagraphStyle\cStyle1">
    <CharacterStyleRange AppliedCharacterStyle="CharacterStyle\k[No character style]">
        <Content>ABC</Content>
     </CharacterStyleRange>
    </ParagraphStyleRange></Story>"##;

    // let story = serde_xml_rs::from_str(std::fs::read_to_string(path).unwrap().as_str()).unwrap();
    let story = serde_xml_rs::from_str(test_xml).unwrap();

    story
}
