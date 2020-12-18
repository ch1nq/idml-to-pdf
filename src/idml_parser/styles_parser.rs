use std::path::Path;
use serde::Deserialize;
use derive_getters::Getters;
// use super::formats::*;

#[derive(Default,Deserialize,Debug,PartialEq,Getters)]
#[serde(rename="idPkg:Styles")]
#[serde(rename_all="PascalCase")]
pub struct IdPkgStyles {
    #[serde(rename="DOMVersion")]
    dom_version: Option<f32>,
    root_object_style_group: Option<RootObjectStyleGroup>,
}

#[derive(Default,Deserialize,Debug,PartialEq,Getters)]
#[serde(rename_all="PascalCase")]
pub struct RootObjectStyleGroup {
    #[serde(rename="Self")]
    id: Option<String>,
    #[serde(rename="ObjectStyle")]
    object_styles: Option<Vec<ObjectStyle>>,
}

#[derive(Default,Deserialize,Debug,PartialEq,Getters)]
#[serde(rename_all="PascalCase")]
pub struct ObjectStyle {
    #[serde(rename="Self")]
    id: Option<String>, 
    name: Option<String>,
    fill_color: Option<String>,
    stroke_weight: Option<f64>,
    stroke_color: Option<String>,
}

pub fn parse_styles_from_path(path: &Path) -> Result<IdPkgStyles, quick_xml::DeError> {
    let xml = std::fs::read_to_string(path).unwrap();
    quick_xml::de::from_str(xml.as_str())
}

impl IdPkgStyles {
    pub fn style_from_id(&self, id: &String) -> Option<&ObjectStyle> {

        // Search through object styles and find one matching the given id
        if let Some(root_style_group) = &self.root_object_style_group {
            if let Some(obj_styles) = &root_style_group.object_styles {
                // Note: Maybe more effecient to implement objstyles as a HashMap, 
                //       to make lookups faster  
                for style in obj_styles {
                    if let Some(style_id) = &style.id {
                        if style_id == id {
                            return Some(style);
                        }
                    } 
                }
            }
        }

        // If we reach this point, no style was found 
        None
    }
}