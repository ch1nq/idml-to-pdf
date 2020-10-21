use std::fs::{File};
use std::io::{BufReader};
use std::path::{Path, PathBuf};
use xml::reader::{EventReader, XmlEvent};
use std::collections::HashMap;
use regex::Regex;

#[derive(Debug)]
pub struct IDMLPackage<'a> {
    dir_path: &'a Path,
    mimetype: String,
    designmap: DesignMap,
    resources: IDMLResources,
    master_spreads: HashMap<String, Spread>,
    spreads: HashMap<String, Spread>,
    stories: HashMap<String, Story>,
    xml: IdmlXml, 
    meta_inf: MetaInf,
}

#[derive(Debug)]
struct DesignMap {
    master_spreads_src: HashMap<String, String>,
    spreads_src: HashMap<String, String>,
    stories_src: HashMap<String, String>
}

#[derive(Debug)]
struct IDMLResources {
    fonts: Vec<String>,
    styles: Vec<String>,
    graphic: Vec<String>,
    preferences: Vec<String>,
}

#[derive(Debug)]
struct Spread {
    id: String,
    pages: Vec<Page>,
}

#[derive(Debug)]
struct Page {
    attributes: Vec<String>,
}

#[derive(Debug)]
struct Story {
    id: String,
    content: String,
}

#[derive(Debug)]
struct IdmlXml {
    backing_story: Vec<String>,
    mapping: Vec<String>,
    tags: Vec<String>,
}

#[derive(Debug)]
struct MetaInf {
    container: String
}

impl IDMLPackage<'_> {
    pub fn from_dir(path: &Path) -> IDMLPackage {
        
        // Parse DesignMap
        let design_map = DesignMap::new(path).unwrap();
        
        // Parse master spreads
        let mut master_spreads = HashMap::new();
        for (id, src) in &design_map.master_spreads_src {
            master_spreads.insert(id.to_string(),Spread::from_file(Path::new(&src)));
        }

        // Parse spreads
        let mut spreads = HashMap::new();
        for (id, src) in &design_map.spreads_src {
            spreads.insert(id.to_string(),Spread::from_file(Path::new(&src)));
        }

        // Parse stories
        let mut stories = HashMap::new();
        for (id, src) in &design_map.stories_src {
            stories.insert(id.to_string(), Story::from_file(Path::new(&src)));
        }

        
        IDMLPackage {
            dir_path: path.clone(),
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
        }
    }
}



impl DesignMap {
    fn new(dir_path: &Path) -> Result<DesignMap, std::io::Error> {
        let mut dm_path = PathBuf::from(dir_path);
        dm_path.push("designmap.xml");
        DesignMap::from_file(&dm_path)
    } 

    fn from_file(path: &Path) -> Result<DesignMap, std::io::Error> {

        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let parser = EventReader::new(reader);

        let mut master_spreads = HashMap::new();
        let mut spreads = HashMap::new();
        let mut stories = HashMap::new();

        let re = Regex::new(r"(\{.+\})(idPkg):(.+)").unwrap();
        
        for e in parser {
            match e {
                // Start node
                Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                    // Capture SOMEVALUE from "{http://ns.adobe.com/AdobeInDesign/idml/1.0/packaging}idPkg:SOMEVALUE"
                    if let Some(caps) = re.captures(name.to_string().as_str()) {
                        match caps.get(3).map_or("", |m| m.as_str()) {
                            "Graphic" => {}
                            "Fonts" => {}
                            "Styles" => {}
                            "Preferences" => {}
                            "Tags" => {}
                            "MasterSpread" => {
                                let attributes:Vec<Vec<String>> = attributes.into_iter().map(|a| {
                                    vec!(a.name.to_string(), a.value)
                                }).collect();
                                
                                let src = &attributes[0][1];
                                let mut full_path = PathBuf::from(path.parent().unwrap());
                                full_path.push(src);
                                let id = Spread::id_from_path(&full_path);

                                master_spreads.insert(id, src.to_owned());
                            }
                            "Spread" => {
                                let attributes:Vec<Vec<String>> = attributes.into_iter().map(|a| {
                                    vec!(a.name.to_string(), a.value)
                                }).collect();
                                
                                let src = &attributes[0][1];
                                let mut full_path = PathBuf::from(path.parent().unwrap());
                                full_path.push(src);
                                let id = Spread::id_from_path(Path::new(src));
                                
                                spreads.insert(id, src.to_owned());
                            }
                            "BackingStory" => {}
                            "Story" => {
                                let attributes:Vec<Vec<String>> = attributes.into_iter().map(|a| {
                                    vec!(a.name.to_string(), a.value)
                                }).collect();
                                
                                let src = &attributes[0][1];
                                let mut full_path = PathBuf::from(path.parent().unwrap());
                                full_path.push(src);
                                let id = Story::id_from_path(Path::new(src));

                                stories.insert(id, full_path.to_str().unwrap().to_string());
                            } 
                            _ => {}
                        }
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                    break;
                }
                _ => {}
            }
        }

        Ok(DesignMap { 
            master_spreads_src: master_spreads,
            spreads_src: spreads,
            stories_src: stories,
        })
    }
}

impl Spread {
    fn from_file(path: &Path) -> Spread {
        let id = Spread::id_from_path(path);

        Spread {
            id: id.to_owned(),
            pages: vec!(Page { 
                attributes: vec!() 
            })
        }
    }

    fn id_from_path(path: &Path) -> String {
        // Get id from the spreads path 
        let re = Regex::new(r"Spread_(.+).xml").unwrap();
        let id = re.captures(path.to_str().unwrap()).unwrap().get(1).unwrap().as_str();

        id.to_string()
    } 
}

impl Story {
    fn from_file(path: &Path) -> Story {
        // Get id from storys path 
        let id = Story::id_from_path(path);
        let content = Story::parse_content(path).unwrap();

        Story {
            id: id.to_owned(),
            content: content
        }
    }

    fn parse_content(path: &Path) -> Result<String, std::io::Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let parser = EventReader::new(reader);

        let mut content = "".to_owned();
        let mut prev_e = None;
        for e in parser {
            match &e {
                Ok(XmlEvent::StartElement { name, .. }) => {
                    match name.to_string().as_str() {
                        "Br" => {
                            // Push newline
                            content.push_str(&"\n");
                        }
                        _ => {}
                    }
                }
                Ok(XmlEvent::Characters(text)) => {
                    match prev_e {
                        Some(Ok(XmlEvent::StartElement { name, .. })) => {
                            match name.to_string().as_str() {
                                "Content" => {
                                    // Push content if we are inside a content block
                                    content.push_str(&text);
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                    break;
    
                }
                _ => {}
            }
            prev_e = Some(e);
        }

        Ok(content.to_string())
    }

    fn id_from_path(path: &Path) -> String {
        // Get id from the spreads path 
        let re = Regex::new(r"Story_(.+).xml").unwrap();
        let id = re.captures(path.to_str().unwrap()).unwrap().get(1).unwrap().as_str();

        id.to_string()
    } 
}