use super::styles::character_style::*;
use super::styles::object_style::*;
use super::styles::paragraph_style::*;
use super::styles::*;
use derive_getters::Getters;
use serde::Deserialize;
use std::path::Path;

#[derive(Default, Deserialize, Debug, PartialEq, Getters)]
#[serde(rename = "idPkg:Styles")]
#[serde(rename_all = "PascalCase")]
pub struct IdPkgStyles {
    #[serde(rename = "DOMVersion")]
    dom_version: Option<f32>,
    root_object_style_group: Option<RootObjectStyleGroup>,
    root_paragraph_style_group: Option<RootParagraphStyleGroup>,
    root_character_style_group: Option<RootCharacterStyleGroup>,
}

impl IdPkgStyles {
    pub fn object_style_from_id(&self, id: &String) -> Option<ObjectStyle> {
        match &self.root_object_style_group {
            Some(root_style_group) => root_style_group.style_from_id(id),
            _ => None,
        }
    }

    pub fn paragraph_style_from_id(&self, id: &String) -> Option<ParagraphStyle> {
        match &self.root_paragraph_style_group {
            Some(root_style_group) => root_style_group.style_from_id(id),
            _ => None,
        }
    }

    pub fn character_style_from_id(&self, id: &String) -> Option<CharacterStyle> {
        match &self.root_character_style_group {
            Some(root_style_group) => root_style_group.style_from_id(id),
            _ => None,
        }
    }
}

pub fn parse_styles_from_path(path: &Path) -> Result<IdPkgStyles, quick_xml::DeError> {
    let xml = std::fs::read_to_string(path).unwrap();
    quick_xml::de::from_str(xml.as_str())
}
