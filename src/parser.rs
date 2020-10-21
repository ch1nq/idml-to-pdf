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
    master_spreads: Vec<Spread>,
    spreads: Vec<Spread>,
    stories: Vec<Story>,
    xml: IdmlXml, 
    meta_inf: MetaInf,
}

#[derive(Debug)]
struct DesignMap {
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
        
        let dm = DesignMap::new(path).unwrap();

        IDMLPackage {
            dir_path: path.clone(),
            mimetype: "MIMETYPE".to_string(), 
            designmap: dm,
            resources: IDMLResources {
                fonts: vec!("Fonts dummy".to_string()),
                styles: vec!("Styles dummy".to_string()),
                graphic: vec!("Graphic dummy".to_string()),
                preferences: vec!("Preferences dummy".to_string()),
            }, 
            master_spreads: vec!( Spread { 
                id: "Id dummy".to_string(),
                pages: vec!( Page { 
                    attributes: vec!("Attribute dummy".to_string())
                })
            }), 
            spreads: vec!(Spread { 
                id: "Id dummy".to_string(),
                pages: vec!( Page { 
                    attributes: vec!("Attribute dummy".to_string())
                })
            }), 
            stories: vec!(Story{
                id: "Id dummy".to_string(),
                content: "Content dummy".to_string()
            }), 
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
                            "MasterSpread" => {}
                            "Spread" => {
                                let attributes:Vec<Vec<String>> = attributes.into_iter().map(|a| {
                                    vec!(a.name.to_string(), a.value)
                                }).collect();
                                
                                let src = &attributes[0][1];
                                let id = Spread::id_from_path(Path::new(src));
                                
                                spreads.insert(id, src.to_owned());
                            }
                            "BackingStory" => {}
                            "Story" => {
                                let attributes:Vec<Vec<String>> = attributes.into_iter().map(|a| {
                                    vec!(a.name.to_string(), a.value)
                                }).collect();
                                
                                let src = &attributes[0][1];
                                let id = Story::id_from_path(Path::new(src));

                                stories.insert(id, src.to_owned());
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
            spreads_src: spreads,
            stories_src: stories,
        })
    }

    fn get_spreads(self) -> Option<Vec<Spread>> {
        unimplemented!();
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

        Story {
            id: id.to_owned(),
            content: "Content dummy".to_string()
        }
    }

    fn id_from_path(path: &Path) -> String {
        // Get id from the spreads path 
        let re = Regex::new(r"Story_(.+).xml").unwrap();
        let id = re.captures(path.to_str().unwrap()).unwrap().get(1).unwrap().as_str();

        id.to_string()
    } 
}

fn _read_idml(_idml_dir:&Path) {
    let file = File::open("files/test-1/Spreads/Spread_uce.xml").unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                if name.to_string() == "Rectangle" {
                    let attributes:Vec<Vec<String>> = attributes.into_iter().map(|a| {vec!(a.name.to_string(), a.value)}).collect();
                    println!("{}: {:?}", name, attributes);
                }
            }
            // Ok(XmlEvent::EndElement {..}) => {}
            Err(e) => {
                println!("Error: {}", e);
                break;

            }
            _ => {}
        }
    }
}