use std::fs::File;
use std::path::Path;

#[derive(Default, Deserialize,Debug)]
#[serde(rename="idPkg:Story")]
#[serde(rename_all="PascalCase")]
pub struct StoryWrapper {
    story: Option<Story>
}

#[derive(Default, Deserialize,Debug)]
#[serde(rename_all="PascalCase")]
pub struct Story {
    //#[serde(rename="Self")]
    //id: String,
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
    #[serde(rename="Content")]
    contents: Option<Vec<Content>>
}

#[derive(Default,Deserialize,Debug)]
#[serde(rename_all="PascalCase")]
pub struct Content {
    #[serde(rename="$value")]
    text: String,
}

pub fn parse_story_from_path(path: &Path) -> StoryWrapper {
    let test_xml = r##"
    <idPkg:Story xmlns:idPkg="http://ns.adobe.com/AdobeInDesign/idml/1.0/packaging" DOMVersion="15.0">
    <Story><ParagraphStyleRange AppliedParagraphStyle="ParagraphStyle\kNormalParagraphStyle"> <CharacterStyleRange AppliedCharacterStyle="CharacterStyle\k[No character style]"
    FillColor="Color\cRed" FontStyle="Italic" PointSize="24"> <Content>ABC</Content>
    <br/>
    </CharacterStyleRange>
    </ParagraphStyleRange>
    <ParagraphStyleRange AppliedParagraphStyle="ParagraphStyle\cStyle1">
    <CharacterStyleRange AppliedCharacterStyle="CharacterStyle\k[No character style]">
        <Content>ABC</Content>
     </CharacterStyleRange>
    </ParagraphStyleRange></Story>
    </idPkg:Story>"##;

    println!("{}", std::fs::read_to_string(path).unwrap().as_str());

    let story = serde_xml_rs::from_str(std::fs::read_to_string(path).unwrap().as_str()).unwrap();
    //let story = serde_xml_rs::from_str(test_xml).unwrap();

    story
}
