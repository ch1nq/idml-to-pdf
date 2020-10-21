use std::path::{Path};
use std::collections::HashMap;


#[derive(Deserialize,Debug)]
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
struct Story {
    id: String,
    content: String,
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
