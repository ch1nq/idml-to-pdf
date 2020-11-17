use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::{fs, io};

use idml_parser::spread_parser;
use idml_parser::story_parser;

#[derive(Deserialize,Debug)]
pub struct IDMLPackage {
    dir_path: String,
    mimetype: String,
    designmap: DesignMap,
    resources: IDMLResources,
    master_spreads: HashMap<String, Spread>,
    spreads: HashMap<String, Spread>,
    stories: Vec<Story>,
    xml: IdmlXml, 
    meta_inf: MetaInf,
}

#[derive(Deserialize,Debug)]
struct DesignMap {
    master_spreads_src: HashMap<String, String>,
    spreads_src: HashMap<String, String>,
    stories_src: HashMap<String, String>
}

#[derive(Deserialize,Debug)]
struct IDMLResources {
    fonts: Vec<String>,
    styles: Vec<String>,
    graphic: Vec<String>,
    preferences: Vec<String>,
}

#[derive(Deserialize,Debug)]
struct Spread {
    id: String,
    pages: Vec<Page>,
}

#[derive(Deserialize,Debug)]
struct Page {
    attributes: Vec<String>,
}

#[derive(Deserialize,Debug)]
struct IdmlXml {
    backing_story: Vec<String>,
    mapping: Vec<String>,
    tags: Vec<String>,
}

#[derive(Deserialize,Debug)]
struct MetaInf {
    container: String
}

impl IDMLPackage {
    pub fn from_dir(path: &Path) -> Result<IDMLPackage, io::Error> {
        
        // Parse DesignMap
        let design_map = DesignMap {
            // Just dummy entries for now 
            master_spreads_src: HashMap::new(),
            spreads_src: HashMap::new(),
            stories_src: HashMap::new()
        };


        // Parse stories
        let mut story_dir = PathBuf::from(path);
        story_dir.push("Stories");
        let stories = (fs::read_dir(story_dir)?).map(|entry| {
            let path = &entry.unwrap().path();
            // println!("{:?}", path);
            let story_wrapper = story_parser::parse_story_from_path(path).unwrap();
            story_wrapper.get_story().unwrap()
        }).collect();
        
        // Parse master spreads
        let master_spreads = HashMap::new();
        
        // Parse spreads
        let mut spread_dir = PathBuf::from(path);
        story_dir.push("Spreads");
        let spreads = (fs::read_dir(spread_dir)?).map(|entry| {
            let path = &entry.unwrap().path();
            // println!("{:?}", path);
            let story_wrapper = spread_parser::parse_spread_from_path(path).unwrap();
            story_wrapper.get_story().unwrap()
        }).collect();
        
        Ok(IDMLPackage {
            dir_path: path.to_str().unwrap().to_string(),
            mimetype: "MIMETYPE".to_string(), 
            designmap: design_map,
            resources: IDMLResources {
                fonts: vec!("Fonts dummy".to_string()),
                styles: vec!("Styles dummy".to_string()),
                graphic: vec!("Graphic dummy".to_string()),
                preferences: vec!("Preferences dummy".to_string()),
            }, 
            master_spreads: master_spreads,
            spreads: spreads,
            stories: stories,
            xml: IdmlXml {
                backing_story: vec!("BackingStory dummy".to_string()),
                mapping: vec!("Mapping dummy".to_string()),
                tags: vec!("Tags dummy".to_string()),
            },
            meta_inf: MetaInf { container: "Container dummy".to_string() }
        })
    }
}