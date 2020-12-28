use serde::Deserialize;
use derive_getters::Getters;
use super::{Style, StyleGroup};

#[derive(Default,Deserialize,Debug,PartialEq,Getters)]
#[serde(rename_all="PascalCase")]
pub struct RootCharacterStyleGroup {
    #[serde(rename="Self")]
    id: Option<String>,
    #[serde(rename="CharacterStyle")]
    character_styles: Option<Vec<CharacterStyle>>,
}

#[derive(Default,Deserialize,Debug,PartialEq,Getters,Clone)]
#[serde(rename_all="PascalCase")]
pub struct CharacterStyle {
    #[serde(rename="Self")]
    id: Option<String>, 
    name: Option<String>,
    fill_color: Option<String>,
    font_style: Option<String>,
    point_size: Option<f64>,
    stroke_weight: Option<f64>,
    stroke_color: Option<String>,
    properties: Option<CharacterProperties>,
}

#[derive(Default,Deserialize,Debug,PartialEq,Getters,Clone)]
#[serde(rename_all="PascalCase")]
pub struct CharacterProperties {
    based_on: Option<String>,
    applied_font: Option<String>,
}

impl Style for CharacterStyle {
    fn get_id(&self) -> &Option<String> { 
        &self.id() 
    }

    fn get_parent_id(&self) -> &Option<String> { 
        match &self.properties {
            Some(properties) => &properties.based_on,
            _ => &None
        }
    }

    fn combine_with_parent(&self, parent: &CharacterStyle) -> CharacterStyle {
        
        // Macros for making a struct calling choose on every field
        macro_rules! choose_fields {
            (
                $child:ident,
                $parent:ident,
                $StructName:ident { $($manual_fields:tt)* },
                $($field:ident),+ $(,)?
            ) => {
                $StructName {
                    $(
                        $field: self.choose($child.$field().clone(), $parent.$field().clone()),
                    )+
                    $($manual_fields)*
                    // .. *parent.clone()
                }
            }
        }
        
        // Get references to property structs
        let combined_properties = match (&self.properties, &parent.properties) {
            (Some(child_props), Some(parent_props)) => 
                Some(
                    choose_fields!(
                        child_props,
                        parent_props,
                        CharacterProperties {
                            // Manually set fields
                            based_on: self.get_parent_id().clone()
                        },
                        // Fields that can be overwritten by child properties
                        applied_font,
                    )
                ),
            _ => None
        };

        choose_fields!(
            self,
            parent,
            CharacterStyle {
                // Manually set fields
                id: self.id.clone(),
                properties: combined_properties
            },
            // Fields that can be overwritten by child
            name,
            fill_color,
            font_style,
            point_size,
            stroke_color,
            stroke_weight,
        )
    }
}

impl StyleGroup<CharacterStyle> for RootCharacterStyleGroup {
    fn get_styles(&self) -> &Option<Vec<CharacterStyle>> { &self.character_styles() }
}
