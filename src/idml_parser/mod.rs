pub mod designmap_parser;
pub mod fonts_parser;
mod formats;
pub mod graphic_parser;
pub mod spread_parser;
#[macro_use]
pub mod styles;
pub mod story_parser;
pub mod styles_parser;

use derive_getters::Getters;
use serde::Deserialize;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::path::{Path, PathBuf};
use std::{fs, io};

use designmap_parser::DesignMap;
use fonts_parser::IdPkgFonts;
use graphic_parser::IdPkgGraphic;
use spread_parser::Spread;
use story_parser::Story;
use styles_parser::IdPkgStyles;

#[derive(Deserialize, Debug, Getters)]
pub struct IDMLPackage {
    dir_path: String,
    mimetype: String,
    designmap: DesignMap,
    resources: IDMLResources,
    master_spreads: HashMap<String, Spread>,
    spreads: HashMap<String, Spread>,
    stories: HashMap<String, Story>,
    xml: IdmlXml,
    meta_inf: MetaInf,
}

#[derive(Deserialize, Debug, Getters)]
pub struct IDMLResources {
    fonts: IdPkgFonts,
    styles: IdPkgStyles,
    graphic: IdPkgGraphic,
    preferences: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Page {
    attributes: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct IdmlXml {
    backing_story: Vec<String>,
    mapping: Vec<String>,
    tags: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct MetaInf {
    container: String,
}

impl IDMLPackage {
    pub fn from_dir(path: &Path) -> Result<IDMLPackage, io::Error> {
        // TODO: Parse each file in parallel for easy performance boost

        // Parse all components of the IDML package
        let design_map = parse_design_map(path).expect("Failed to parse designmap");
        let resources = parse_resources(path).expect("Failed to parse resources");
        let master_spreads = parse_master_spreads(path).expect("Failed to parse master spreads");
        let spreads = parse_spreads(path).expect("Failed to parse spreads");
        let stories = parse_stories(path).expect("Failed to parse stories");

        // Combine everything
        let idml_package = IDMLPackage {
            dir_path: path.to_str().unwrap().to_string(),
            mimetype: "MIMETYPE".to_string(),
            designmap: design_map,
            resources: resources,
            master_spreads: master_spreads,
            spreads: spreads,
            stories: stories,
            xml: IdmlXml {
                backing_story: vec!["BackingStory dummy".to_string()],
                mapping: vec!["Mapping dummy".to_string()],
                tags: vec!["Tags dummy".to_string()],
            },
            meta_inf: MetaInf {
                container: "Container dummy".to_string(),
            },
        };

        Ok(idml_package)
    }

    pub fn master_spread_with_id(&self, id: &str) -> Option<&Spread> {
        self.master_spreads.get(id)
    }
}

fn parse_design_map(path: &Path) -> Result<DesignMap, quick_xml::DeError> {
    let mut xml_path = PathBuf::from(path);
    xml_path.push("designmap.xml");
    designmap_parser::parse_designmap_from_path(&xml_path)
}

fn parse_resources(path: &Path) -> Result<IDMLResources, io::Error> {
    let mut resource_dir = PathBuf::from(path);
    resource_dir.push("Resources");

    // Fonts
    resource_dir.push("Fonts.xml");
    let fonts =
        fonts_parser::parse_fonts_from_path(&resource_dir).expect("Failed to parse Fonts.xml");
    resource_dir.pop();

    // Styles
    resource_dir.push("Styles.xml");
    let styles =
        styles_parser::parse_styles_from_path(&resource_dir).expect("Failed to parse Styles.xml");
    resource_dir.pop();

    // Graphic
    resource_dir.push("Graphic.xml");
    let graphic = graphic_parser::parse_graphic_from_path(&resource_dir)
        .expect("Failed to parse Graphic.xml");
    resource_dir.pop();

    // Preferences
    resource_dir.push("Preferences.xml");
    let preferences = vec!["Preferences dummy".to_string()];
    resource_dir.pop();

    let resources = IDMLResources {
        fonts: fonts,
        styles: styles,
        graphic: graphic,
        preferences: preferences,
    };

    Ok(resources)
}

fn parse_stories(path: &Path) -> Result<HashMap<String, Story>, io::Error> {
    let mut story_dir = PathBuf::from(path);
    story_dir.push("Stories");

    if let Ok(dir) = fs::read_dir(story_dir) {
        let stories = dir.map(|entry| {
            let path = &entry.unwrap().path();
            let story_wrapper = story_parser::parse_story_from_path(path).unwrap();
            let story = story_wrapper.get_story().expect("No story found");
            (story.id().clone(), story)
        });
        Ok(HashMap::from_iter(stories))
    } else {
        Ok(HashMap::default())
    }
}

fn parse_master_spreads(path: &Path) -> Result<HashMap<String, Spread>, io::Error> {
    let mut spread_dir = PathBuf::from(path);
    spread_dir.push("MasterSpreads");

    let master_spreads = (fs::read_dir(spread_dir)?).map(|entry| {
        let path = &entry.unwrap().path();
        let spread_wrapper = spread_parser::parse_spread_from_path(path).unwrap();
        let spread = spread_wrapper.get_spread();
        (spread.id().clone().unwrap(), spread)
    });
    Ok(HashMap::from_iter(master_spreads))
}

fn parse_spreads(path: &Path) -> Result<HashMap<String, Spread>, io::Error> {
    let mut spread_dir = PathBuf::from(path);
    spread_dir.push("Spreads");

    let spreads = (fs::read_dir(spread_dir)?).map(|entry| {
        let path = &entry.unwrap().path();
        let spread_wrapper = spread_parser::parse_spread_from_path(path).unwrap();
        let spread = spread_wrapper.get_spread();
        (spread.id().clone().unwrap(), spread)
    });
    Ok(HashMap::from_iter(spreads))
}
