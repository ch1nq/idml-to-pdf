use std::path::Path;
use quick_xml::de::{from_str, DeError};

#[derive(Default, Deserialize,Debug)]
#[serde(rename="idPkg:Story")]
#[serde(rename_all="PascalCase")]
pub struct SpreadWrapper {
    #[serde(rename="DOMVersion")]
    dom_version: Option<f32>,
    spread: Option<Spread>,
}

#[derive(Default, Deserialize,Debug)]
// #[derive(Default,Debug)]
#[serde(rename_all="PascalCase")]
pub struct Spread {
    #[serde(rename="Self")]
    id: String,
    // page_count: u32, 
    #[serde(rename = "$value")]
    contents: Vec<Option<PageContent>>
}

#[derive(Default,Deserialize,Debug)]
#[serde(rename_all="PascalCase")]
pub struct Page {
    // #[serde(rename="Self")]
    // id: String,
    // // geometric_bounds: Vec<i32>,
    applied_master: Option<String>,
    // applied_paragraph_style: Option<String>,
}

// #[derive(Debug)]
#[derive(Deserialize,Debug)]
// #[serde(untagged)]
pub enum PageContent {
    FlattenerPreference(FlattenerPreference),
    Page(Page),
    Rectangle(Rectangle),
    Polygon(Polygon),
    Oval(Oval),
    Group(Group),
    TextFrame(TextFrame),
    Other
}

impl Default for PageContent {
    fn default() -> Self {
        PageContent::Other
    }
}

#[derive(Deserialize,Debug)]
#[serde(rename_all="PascalCase")]
pub struct FlattenerPreference {
    // #[serde(rename="Self")]
    // id: String,
    // fill_color: Option<String>,
}

#[derive(Deserialize,Debug)]
#[serde(rename_all="PascalCase")]
pub struct Rectangle {
    #[serde(rename="Self")]
    id: String,
    fill_color: Option<String>,
    properties: Option<Properties>,
    // text_wrap_preference: Option<String>
}

#[derive(Deserialize,Debug)]
#[serde(rename_all="PascalCase")]
pub struct Polygon {
    #[serde(rename="Self")]
    id: String,
    // fill_color: Option<String>,
}

#[derive(Deserialize,Debug)]
#[serde(rename_all="PascalCase")]
pub struct Oval {
    #[serde(rename="Self")]
    id: String,
    // fill_color: Option<String>,
}

#[derive(Deserialize,Debug)]
#[serde(rename_all="PascalCase")]
pub struct Group {
    #[serde(rename="Self")]
    id: String,
    // fill_color: Option<String>,
    // #[serde(rename="$value")]
    // contents: Vec<PageContent>
}

#[derive(Deserialize,Debug)]
#[serde(rename_all="PascalCase")]
pub struct TextFrame {
    #[serde(rename="Self")]
    id: String,
    fill_color: Option<String>,
    parent_story: Option<String>,
    previous_text_frame: Option<String>,
    next_text_frame: Option<String>,
}

#[derive(Deserialize,Debug)]
#[serde(rename_all="PascalCase")]
pub struct Properties {
    applied_font: Option<String>,
    path_geometry: Option<PathGeometry>
}

#[derive(Deserialize,Debug)]
#[serde(rename_all="PascalCase")]
pub struct PathGeometry {
    geometry_path_type: GeometryPathType
}

#[derive(Deserialize,Debug)]
#[serde(rename_all="PascalCase")]
pub struct GeometryPathType {
    path_open: bool,
    #[serde(rename="PathPointArray")]
    path_point_arrays: Vec<PathPointArray>
}

#[derive(Deserialize,Debug)]
#[serde(rename_all="PascalCase")]
pub struct PathPointArray {
    #[serde(rename="$value")]
    path_point_array: Vec<PathPointType>
}

#[derive(Deserialize,Debug)]
#[serde(rename_all="PascalCase")]
pub struct PathPointType {
    anchor: Option<String>,
    left_direction: Option<String>,
    right_direction: Option<String>
}



pub fn parse_spread_from_path(path: &Path) -> Result<SpreadWrapper, DeError> {
    let xml = std::fs::read_to_string(path).unwrap();
    // serde_xml_rs::from_str(xml.as_str())
    from_str(xml.as_str())
}

impl SpreadWrapper {
    pub fn get_spread(self) -> Option<Spread> {
        self.spread
    }
}

impl Spread {
    // pub fn get_id(self) -> String {
    //     self.id
    // }
}
