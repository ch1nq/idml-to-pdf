use std::path::Path;
use quick_xml::de::{DeError};
use serde::Deserialize;

#[derive(Default, Deserialize,Debug,PartialEq)]
#[serde(rename="idPkg:MasterSpread")]
#[serde(rename_all="PascalCase")]
pub struct IdPkgSpread {
    #[serde(rename="DOMVersion")]
    dom_version: Option<f32>,
    #[serde(rename="$value")]
    spread: Spread,
}

#[derive(Default, Deserialize,Debug,PartialEq)]
#[serde(rename="MasterSpread")]
#[serde(rename_all="PascalCase")]
pub struct Spread {
    #[serde(rename="Self")]
    id: Option<String>,
    flattener_override: Option<String>,
    // #[serde(deserialize_with="deserialize_space_seperated_opt_vec")]
    // item_transform: Option<Vec<f64>>,
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

impl Spread {
    pub fn pages(&self) -> Vec<Option<&Page>> {
        let pages: Vec<Option<&Page>> = self.contents()
        .into_iter()
        .map(|content| 
            match content { 
                SpreadContent::Page(p) => Some(p),
                _ => None
            }
        ).collect();

        // pages.retain(|&c| c.is_some());

        pages
    }

    pub fn contents(&self) -> &Vec<SpreadContent> {
        &self.contents
    }
}

#[derive(Deserialize,Debug,PartialEq)]
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

#[derive(Default,Deserialize,Debug,PartialEq)]
#[serde(rename_all="PascalCase")]
pub struct Page {
    applied_alternate_layout: Option<String>,
    applied_master: Option<String>,
    applied_trap_preset: Option<String>,
    #[serde(deserialize_with="deserialize_space_seperated_vec")]
    geometric_bounds: Vec<f64>,
    grid_starting_point: Option<String>,
    #[serde(deserialize_with="deserialize_space_seperated_opt_vec")]
    item_transform: Option<Vec<f64>>,
    layout_rule: Option<String>,
    #[serde(deserialize_with="deserialize_space_seperated_opt_vec")]
    master_page_transform: Option<Vec<f64>>,
    name: Option<String>,
    optional_page: Option<bool>,
    override_list: Option<String>, // Should be Option<Vec<String>>
    page_color: Option<PageColorOptions>,
    snapshot_blending_mode: Option<String>, 
    tab_order: Option<String>,
    use_master_grid: bool,
    margin_preference: MarginPreference,
    properties: Properties
}

impl Page {
    pub fn geometric_bounds(&self) -> &Vec<f64> {
        &self.geometric_bounds
    } 
}

#[derive(Deserialize,Debug,PartialEq)]
pub enum PageColorOptions {
    UseMasterColor,
    Color(Color),
    #[serde(other)]
    Nothing
}

#[derive(Default,Deserialize,Debug,PartialEq)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64
}

#[derive(Default,Deserialize,Debug,PartialEq)]
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
    columns_positions: Vec<f64>,
}

#[derive(Default,Deserialize,Debug,PartialEq)]
#[serde(rename_all="PascalCase")]
pub struct FlattenerPreference {
    // #[serde(rename="Self")]
    // id: String,
    // fill_color: Option<String>,
}

#[derive(Default,Deserialize,Debug,PartialEq)]
#[serde(rename_all="PascalCase")]
pub struct Rectangle {
    #[serde(rename="Self")]
    id: String,
    fill_color: Option<String>,
    properties: Option<Properties>,
    // text_wrap_preference: Option<String>
}

#[derive(Default,Deserialize,Debug,PartialEq)]
#[serde(rename_all="PascalCase")]
pub struct Polygon {
    #[serde(rename="Self")]
    id: String,
    // fill_color: Option<String>,
}

#[derive(Default,Deserialize,Debug,PartialEq)]
#[serde(rename_all="PascalCase")]
pub struct Oval {
    #[serde(rename="Self")]
    id: String,
    // fill_color: Option<String>,
}

#[derive(Default,Deserialize,Debug,PartialEq)]
#[serde(rename_all="PascalCase")]
pub struct Group {
    #[serde(rename="Self")]
    id: String,
    // fill_color: Option<String>,
    // #[serde(rename="$value")]
    // contents: Vec<PageContent>
}

#[derive(Default,Deserialize,Debug,PartialEq)]
#[serde(rename_all="PascalCase")]
pub struct TextFrame {
    #[serde(rename="Self")]
    id: String,
    fill_color: Option<String>,
    parent_story: Option<String>,
    previous_text_frame: Option<String>,
    next_text_frame: Option<String>,
}

#[derive(Default,Deserialize,Debug,PartialEq)]
#[serde(rename_all="PascalCase")]
pub struct Properties {
    applied_font: Option<String>,
    path_geometry: Option<PathGeometry>,
    // descriptor: Option<Descriptor>, 
}

// #[derive(Default,Deserialize,Debug)]
// #[serde(rename_all="PascalCase")]
// pub struct Descriptor {
//     #[serde(rename="$value")]
//     list_items: Vec<ListItem>
// }

// #[derive(Default,Deserialize,Debug)]
// #[serde(rename_all="PascalCase")]
// pub struct ListItem {
//     // #[serde(rename="type")]
//     // list_item_type: ListItemType,
//     #[serde(rename="$value")]
//     value: String
// }

// #[derive(Deserialize,Debug)]
// #[serde(rename_all="lowercase")]
// pub enum ListItemType {
//     Enumeration,
//     Boolean,
//     Long,
//     #[serde(other)]
//     String,
// }

// impl Default for ListItemType {
//     fn default() -> Self {
//         Self::String
//     }
// }

#[derive(Default,Deserialize,Debug,PartialEq)]
#[serde(rename_all="PascalCase")]
pub struct PathGeometry {
    geometry_path_type: GeometryPathType
}

#[derive(Default,Deserialize,Debug,PartialEq)]
#[serde(rename_all="PascalCase")]
pub struct GeometryPathType {
    path_open: bool,
    #[serde(rename="PathPointArray")]
    path_point_arrays: Vec<PathPointArray>
}

#[derive(Default,Deserialize,Debug,PartialEq)]
#[serde(rename_all="PascalCase")]
pub struct PathPointArray {
    #[serde(rename="$value")]
    path_point_array: Vec<PathPointType>
}

#[derive(Default,Deserialize,Debug,PartialEq)]
#[serde(rename_all="PascalCase")]
pub struct PathPointType {
    #[serde(deserialize_with="deserialize_space_seperated_opt_vec")]
    anchor: Option<Vec<f32>>,
    #[serde(deserialize_with="deserialize_space_seperated_opt_vec")]
    left_direction: Option<Vec<f32>>,
    #[serde(deserialize_with="deserialize_space_seperated_opt_vec")]
    right_direction: Option<Vec<f32>>
}

pub fn parse_spread_from_path(path: &Path) -> Result<IdPkgSpread, DeError> {
    let xml = std::fs::read_to_string(path).unwrap();
    quick_xml::de::from_str(xml.as_str())
}

impl IdPkgSpread {
    pub fn get_spread(self) -> Spread {
        self.spread
    }
}

fn deserialize_space_seperated_vec<'de, D, N>(deserializer: D) -> Result<Vec<N>, D::Error>
where
    D: serde::de::Deserializer<'de>,
    N: std::str::FromStr + std::fmt::Debug,
    <N as std::str::FromStr>::Err: std::fmt::Debug,
{
    let s: std::borrow::Cow<str> = serde::de::Deserialize::deserialize(deserializer)?;
    let vec = s.split(' ').map(|e| 
        e.to_string().parse::<N>().expect(format!("Failed to parse string '{}' into number", e).as_str())
    ).collect();

    Ok(vec)
}

fn deserialize_space_seperated_opt_vec<'de, D, N>(deserializer: D) -> Result<Option<Vec<N>>, D::Error>
where
    D: serde::de::Deserializer<'de>,
    N: std::str::FromStr + std::fmt::Debug,
    <N as std::str::FromStr>::Err: std::fmt::Debug,
{
    // FIXME: Cannot handle cases where field does not exist
    match deserialize_space_seperated_vec(deserializer) {
        Ok(v) => Ok(Some(v)),
        Err(e) => Err(e)
    }
}