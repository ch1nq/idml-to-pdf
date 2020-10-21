use std::fs::{File};
use std::io::{BufReader};
use std::path::{Path};
use xml::reader::{EventReader, XmlEvent};

#[derive(Debug)]
pub struct IDMLPackage<'a> {
    dir_path: &'a Path,
    mimetype: String,
    designmap: Vec<String>,
    resources: IDMLResources,
    master_spreads: Vec<Spread>,
    spreads: Vec<Spread>,
    stories: Vec<Story>,
    xml: IdmlXml, 
    meta_inf: MetaInf,
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
        IDMLPackage {
            dir_path: path.clone(),
            mimetype: "MIMETYPE".to_string(), 
            designmap: vec!("Designmap dummy".to_string()),  
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


fn read_idml(idml_dir:&Path) {
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
