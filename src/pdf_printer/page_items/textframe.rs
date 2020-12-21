// use printpdf::{PdfDocumentReference, Point, Pt, Line};
// use printpdf::indices::{PdfLayerIndex, PdfPageIndex};
// use super::transforms::Transform;
// use crate::idml_parser::IDMLResources;
use crate::idml_parser::spread_parser::*;

impl TextFrame {
    pub fn render_story(&self) -> Result<(), String> {
        print!("Id: {}  ", &self.id());
        if let Some(id) = &self.parent_story() {
            print!("ParentStory: {}  ", id);
        }
        if let Some(prev) = &self.previous_text_frame() {
            print!("Prev: {}  ", prev);
        }
        if let Some(next) = &self.next_text_frame() {
            print!("Next: {}  ", next);
        }
        println!("");

        Ok(())
    }
}
