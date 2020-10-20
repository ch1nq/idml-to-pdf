use std::fs::{File, remove_dir_all};
use std::io::{BufReader};
use std::path::{Path};
use xml::reader::{EventReader, XmlEvent};

struct IDMLPackage {
    DirPath: Path,
    Mimetype: String,
    Designmap: Vec<String>,
    Resources: IDMLResources,
    MasterSpreads: Vec<Spread>,
    Spreads: Vec<Spread>,
    Stories: Vec<Story>,
}

struct IDMLResources {
    Fonts: Vec<String>,
    Styles: Vec<String>,
    Graphic: Vec<String>,
    Preferences: Vec<String>,
}

struct Spread {
    Id: String,
}

struct Story {
    Id: String,
    Content: String,
}

struct IDML_XML {
    BackingStory: Vec<String>,
    Mapping: Vec<String>,
    Tags: Vec<String>,
}

impl IDMLPackage {
    pub fn from_dir() -> IDMLFile {

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
