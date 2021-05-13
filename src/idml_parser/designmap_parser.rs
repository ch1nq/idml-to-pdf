use derive_getters::Getters;
use quick_xml::DeError;
use regex::Regex;
use serde::Deserialize;
use std::path::Path;

#[derive(Default, Deserialize, Debug, PartialEq, Getters)]
#[serde(rename_all = "PascalCase")]
struct Document {
    #[serde(rename = "$value")]
    designmap_content: Vec<DesignMapContent>,
}

#[derive(Deserialize, Debug, PartialEq)]
enum DesignMapContent {
    #[serde(rename = "idPkg:MasterSpread")]
    MasterSpreadSrc(SrcWrapper),
    #[serde(rename = "idPkg:Spread")]
    SpreadSrc(SrcWrapper),
    #[serde(rename = "idPkg:Story")]
    StorySrc(SrcWrapper),
    #[serde(other)]
    NotImplemented,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct SrcWrapper {
    src: String,
}

#[derive(Default, Deserialize, Debug, PartialEq, Getters)]
pub struct DesignMap {
    master_spread_ids: Vec<String>,
    spread_ids: Vec<String>,
    story_ids: Vec<String>,
}

pub fn parse_designmap_from_path(path: &Path) -> Result<DesignMap, DeError> {
    let xml = std::fs::read_to_string(path).unwrap();
    let document: Document = quick_xml::de::from_str(xml.as_str())?;

    let re = Regex::new(r"_(.+)\.xml").unwrap();
    let path_to_id = |path| re.captures(path).unwrap()[1].to_string();

    let master_spread_ids = document
        .designmap_content()
        .iter()
        .filter_map(|x| match x {
            DesignMapContent::MasterSpreadSrc(src) => Some(src),
            _ => None,
        })
        .map(|x| path_to_id(&x.src))
        .collect();

    let story_ids = document
        .designmap_content()
        .iter()
        .filter_map(|x| match x {
            DesignMapContent::StorySrc(src) => Some(src),
            _ => None,
        })
        .map(|x| path_to_id(&x.src))
        .collect();

    let spread_ids = document
        .designmap_content()
        .iter()
        .filter_map(|x| match x {
            DesignMapContent::SpreadSrc(src) => Some(src),
            _ => None,
        })
        .map(|x| path_to_id(&x.src))
        .collect();

    let design_map = DesignMap {
        master_spread_ids,
        spread_ids,
        story_ids,
    };

    Ok(design_map)
}
