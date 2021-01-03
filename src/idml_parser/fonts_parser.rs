use std::path::Path;
use serde::Deserialize;
use derive_getters::Getters;
// use super::formats::*;

#[derive(Default,Deserialize,Debug,PartialEq,Getters)]
#[serde(rename="idPkg:Fonts")]
#[serde(rename_all="PascalCase")]
pub struct IdPkgFonts {
    #[serde(rename="DOMVersion")]
    dom_version: Option<f32>,
    #[serde(rename="FontFamily")]
    font_families: Vec<FontFamily>,
    #[serde(rename="CompositeFont")]
    composite_fonts: Vec<CompositeFont>,
}

#[derive(Default,Deserialize,Debug,PartialEq,Getters)]
#[serde(rename_all="PascalCase")]
pub struct FontFamily {
    #[serde(rename="Self")]
    id: String,
    name: Option<String>,
    #[serde(rename="Font")]
    fonts: Vec<Font>,
}

#[derive(Default,Deserialize,Debug,PartialEq,Getters)]
#[serde(rename_all="PascalCase")]
pub struct Font {
    #[serde(rename="Self")]
    id: String,
    font_family: String,
    font_style_name: String,
    font_style_name_native: String,
    font_type: FontTypes,
    full_name: String,
    full_name_native: String,
    name: String,
    platform_name: String,
    post_script_name: String,
    status: FontStatus,
    version: String,
    writing_script: i8,
}

#[derive(Deserialize,Debug,PartialEq)]
pub enum FontTypes {
    Type1,
    TrueType,
    CID,
    ATC,
    Bitmap,
    OCF,
    OpenTypeCFF,
    OpenTypeCID,
    OpenTypeTT,
    #[serde(other)]
    Unknown
}

impl Default for FontTypes {
    fn default() -> Self { FontTypes::Unknown }
}

#[derive(Deserialize,Debug,PartialEq)]
pub enum FontStatus {
    Installed,
    NotAvailable,
    Fauxed,
    Substituted,
    #[serde(other)]
    Unknown
}

impl Default for FontStatus {
    fn default() -> Self { FontStatus::Unknown }
}


#[derive(Default,Deserialize,Debug,PartialEq,Getters)]
#[serde(rename_all="PascalCase")]
pub struct CompositeFont {
    // #[serde(rename="Self")]
    // id: String,
    // name: Option<String>,
    // #[serde(rename="Font")]
    // fonts: Option<Vec<Font>>,
}

pub fn parse_fonts_from_path(path: &Path) -> Result<IdPkgFonts, quick_xml::DeError> {
    let xml = std::fs::read_to_string(path).unwrap();
    quick_xml::de::from_str(xml.as_str())
}

impl IdPkgFonts {
    pub fn font_from_id(&self, id: &String) -> Option<&Font> {

        // Search through font families and find one matching the given id
        // Note: Maybe more effecient to implement font families as a HashMap, 
        //       to make lookups faster  
        for family in self.font_families() {
            for font in family.fonts() {
                if font.id() == id {
                    return Some(font);
                }
            }
        }
        
        // If we reach this point, no style was found 
        None
    }
}