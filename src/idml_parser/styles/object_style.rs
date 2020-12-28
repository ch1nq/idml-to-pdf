use serde::Deserialize;
use derive_getters::Getters;
use super::{Style, StyleGroup};

#[derive(Default,Deserialize,Debug,PartialEq,Getters)]
#[serde(rename_all="PascalCase")]
pub struct RootObjectStyleGroup {
    #[serde(rename="Self")]
    id: Option<String>,
    #[serde(rename="ObjectStyle")]
    object_styles: Option<Vec<ObjectStyle>>,
}

#[derive(Default,Deserialize,Debug,PartialEq,Getters,Clone)]
#[serde(rename_all="PascalCase")]
pub struct ObjectStyle {
    #[serde(rename="Self")]
    id: Option<String>, 
    name: Option<String>,
    fill_color: Option<String>,
    stroke_weight: Option<f64>,
    stroke_color: Option<String>,
    properties: Option<ObjectProperties>,
}

#[derive(Default,Deserialize,Debug,PartialEq,Getters,Clone)]
#[serde(rename_all="PascalCase")]
pub struct ObjectProperties {
    based_on: Option<String>,
    some_random_field: Option<String>, // TODO: Add actual fields for object properties
}

impl Style for ObjectStyle {
    fn get_id(&self) -> &Option<String> { 
        &self.id() 
    }
    
    fn get_parent_id(&self) -> &Option<String> { 
        match &self.properties {
            Some(properties) => &properties.based_on,
            _ => &None
        }
    }

    fn combine_with_parent(&self, parent: &ObjectStyle) -> ObjectStyle {
        
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
                        ObjectProperties {
                            // Manually set fields
                            based_on: self.get_parent_id().clone()
                        },
                        // Fields that can be overwritten by child properties
                        some_random_field
                    )
                ),
            _ => None
        };

        choose_fields!(
            self,
            parent,
            ObjectStyle {
                // Manually set fields
                id: self.id.clone(),
                properties: combined_properties
            },
            // Fields that can be overwritten by child
            name,
            fill_color,
            stroke_color,
            stroke_weight,
        )
    }
}

impl StyleGroup<ObjectStyle> for RootObjectStyleGroup {
    fn get_styles(&self) -> &Option<Vec<ObjectStyle>> { &self.object_styles() }
}
