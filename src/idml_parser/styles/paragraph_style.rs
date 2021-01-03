use super::{Style, StyleGroup};
use derive_getters::Getters;
use serde::Deserialize;
use std::default::Default;

#[derive(Default, Deserialize, Debug, PartialEq, Getters)]
#[serde(rename_all = "PascalCase")]
pub struct RootParagraphStyleGroup {
    #[serde(rename = "Self")]
    id: Option<String>,
    #[serde(rename = "ParagraphStyle")]
    paragraph_styles: Option<Vec<ParagraphStyle>>,
}

#[derive(Default, Deserialize, Debug, PartialEq, Getters, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ParagraphStyle {
    #[serde(rename = "Self")]
    id: Option<String>,
    name: Option<String>,
    fill_color: Option<String>,
    font_style: Option<String>,
    stroke_weight: Option<f64>,
    stroke_color: Option<String>,
    point_size: Option<f64>,
    properties: Option<ParagraphProperties>,
}

#[derive(Default, Deserialize, Debug, PartialEq, Getters, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ParagraphProperties {
    based_on: Option<String>,
    applied_font: Option<String>,
}

impl Style for ParagraphStyle {
    fn get_id(&self) -> &Option<String> {
        &self.id()
    }

    fn get_parent_id(&self) -> &Option<String> {
        match &self.properties {
            Some(properties) => &properties.based_on,
            _ => &None,
        }
    }

    fn combine_with_parent(&self, parent: &ParagraphStyle) -> ParagraphStyle {
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
            (Some(child_props), Some(parent_props)) => Some(choose_fields!(
                child_props,
                parent_props,
                ParagraphProperties {
                    // Manually set fields
                    based_on: self.get_parent_id().clone()
                },
                // Fields that can be overwritten by child properties
                applied_font,
            )),
            _ => None,
        };

        choose_fields!(
            self,
            parent,
            ParagraphStyle {
                // Manually set fields
                id: self.id.clone(),
                properties: combined_properties
            },
            // Fields that can be overwritten by child
            name,
            fill_color,
            font_style,
            stroke_color,
            stroke_weight,
            point_size,
        )
    }
}

impl StyleGroup<ParagraphStyle> for RootParagraphStyleGroup {
    fn get_styles(&self) -> &Option<Vec<ParagraphStyle>> {
        &self.paragraph_styles()
    }
}
