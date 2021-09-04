use super::*;
use super::commom_text_properties::*;
use derive_getters::Getters;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Default, Deserialize, Debug, PartialEq, Getters)]
#[serde(rename_all = "PascalCase")]
pub struct RootCharacterStyleGroup {
    #[serde(rename = "Self")]
    id: Option<String>,
    #[serde(rename = "CharacterStyle")]
    character_styles: Option<Vec<CharacterStyle>>,
}

#[derive(Default, Deserialize, Debug, PartialEq, Getters, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CharacterStyle {
    #[serde(alias="Self")]
    id: Option<String>, 
    name: Option<String>, 
    properties: Option<Properties>,

    #[serde(flatten)]
    // ctp_fields: HashMap<CTPKey, CTPValue>
    ctp_fields: CTPMap
}

impl_common_text_properties!(CharacterStyle);

impl Style for CharacterStyle {
    fn get_id(&self) -> &Option<String> {
        &self.id()
    }

    fn get_name(&self) -> &Option<String> {
        &self.name()
    }

    fn get_parent_id(&self) -> &Option<String> {
        match &self.properties {
            Some(properties) => &properties.based_on(),
            _ => &None,
        }
    }
}

impl StyleGroup<CharacterStyle> for RootCharacterStyleGroup {
    fn get_styles(&self) -> &Option<Vec<CharacterStyle>> {
        &self.character_styles()
    }
}
