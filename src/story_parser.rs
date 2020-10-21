use std::fs::File;
use std::path::Path;

#[derive(Deserialize,Debug)]
pub struct Story {
    // #[serde(rename="Self")]
    // id: String,
    #[serde(default)]
    UserText: bool,

    #[serde(default)]
    StoryTitle: String,
    
    #[serde(default)]
    StoryPreference: StoryPreference,

    #[serde(default)]
    InCopyExportOption: InCopyExportOption,

    #[serde(default)]
    #[serde(rename="ParagraphStyleRange")]
    ParagraphStyleRanges: Vec<ParagraphStyleRange>
}

#[derive(Deserialize,Debug)]
pub struct StoryPreference {
    #[serde(default)]
    OpticalMarginAlignment: bool,
}

impl Default for StoryPreference {
    fn default() -> Self{
        StoryPreference {
            OpticalMarginAlignment: true
        }
    }
}

#[derive(Deserialize,Debug)]
pub struct InCopyExportOption {
    #[serde(default)]
    IncludeGraphicProxies: bool,
}

impl Default for InCopyExportOption {
    fn default() -> Self{
        InCopyExportOption {
            IncludeGraphicProxies: false
        }
    }
}

#[derive(Deserialize,Debug)]
pub struct ParagraphStyleRange {
    #[serde(default)]
    AppliedParagraphStyle: String,

    #[serde(default)]
    Justification: String,
    
    #[serde(default)]
    #[serde(rename="CharacterStyleRange")]
    CharacterStyleRanges: Vec<CharacterStyleRange>
}

#[derive(Deserialize,Debug)]
pub struct CharacterStyleRange {
    #[serde(default)]
    AppliedCharacterStyle: String,

    #[serde(default)]
    FillColor: String,

    #[serde(default)]
    FontStyle: String, 

    #[serde(default)]
    PointSize: String,
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

     let story = serde_xml_rs::from_str(std::fs::read_to_string(path).unwrap().as_str()).unwrap();
    //let story = serde_xml_rs::from_str(test_xml).unwrap();

    story
}
