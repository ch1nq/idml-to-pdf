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
    flattener_override: Option<String>,
    #[serde(deserialize_with="deserialize_space_seperated_opt_vec")]
    item_transform: Option<Vec<f64>>,
    show_master_items: Option<bool>,
    page_count: Option<i32>,
    binding_location: Option<i32>,
    allow_page_shuffle: Option<bool>,
    applied_master: Option<String>,
    name: Option<String>,
    name_prefix: Option<String>,
    base_name: Option<String>,
    page_transition_direction: Option<String>,
    page_transition_type: Option<String>,
    // #[serde(deserialize_with="deserialize_space_seperated_opt_vec")]
    // page_color: Option<Vec<f64>>,
    #[serde(rename = "$value")]
    contents: Vec<SpreadContent>
}

#[derive(Deserialize,Debug)]
pub enum SpreadContent {
    FlattenerPreference(FlattenerPreference),
    Page(Page),
    Rectangle(Rectangle),
    Polygon(Polygon),
    Oval(Oval),
    Group(Group),
    TextFrame(TextFrame),
    #[serde(other)]
    NotImplementedYet
}

#[derive(Default,Deserialize,Debug)]
#[serde(rename_all="PascalCase")]
pub struct Page {
    name: String,
    applied_trap_preset: String,
    applied_master: String,
    override_list: Option<String>,
    tab_order: Option<String>,
    grid_starting_point: Option<String>,
    use_master_grid: bool,
    margin_preference: MarginPreference
}

#[derive(Default,Deserialize,Debug)]
#[serde(rename_all="PascalCase")]
pub struct MarginPreference {
    column_count: i32,
    column_gutter: i32,
    top: i32,
    bottom: i32,
    left: i32,
    right: i32,
    column_direction: String,
    #[serde(deserialize_with="deserialize_space_seperated_vec")]
    columns_positions: Vec<i32>,
}

fn deserialize_space_seperated_vec<'de, D, N>(deserializer: D) -> Result<Vec<N>, D::Error>
where
    D: serde::de::Deserializer<'de>,
    N: std::str::FromStr + std::fmt::Debug,
    <N as std::str::FromStr>::Err: std::fmt::Debug,
{
    let s: std::borrow::Cow<str> = serde::de::Deserialize::deserialize(deserializer)?;
    let vec = s.split(' ').map(|e| 
        e.to_string().parse::<N>().expect(format!("Failed to parse string '{}'", e).as_str())
    ).collect();

    Ok(vec)
}

fn deserialize_space_seperated_opt_vec<'de, D, N>(deserializer: D) -> Result<Option<Vec<N>>, D::Error>
where
    D: serde::de::Deserializer<'de>,
    N: std::str::FromStr + std::fmt::Debug,
    <N as std::str::FromStr>::Err: std::fmt::Debug,
{
    match deserialize_space_seperated_vec(deserializer) {
        Ok(v) => Ok(Some(v)),
        Err(e) => Err(e)
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
    #[serde(deserialize_with="deserialize_space_seperated_opt_vec")]
    anchor: Option<Vec<f32>>,
    #[serde(deserialize_with="deserialize_space_seperated_opt_vec")]
    left_direction: Option<Vec<f32>>,
    #[serde(deserialize_with="deserialize_space_seperated_opt_vec")]
    right_direction: Option<Vec<f32>>
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
