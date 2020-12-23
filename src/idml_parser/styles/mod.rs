pub mod character_style;
pub mod paragraph_style;
pub mod object_style;

pub trait Style {
    fn get_id(&self) -> &Option<String>;
}

pub trait StyleGroup<T: Style> {
    fn get_styles(&self) -> &Option<Vec<T>>;
    fn style_from_id(&self, id: &String) -> Option<&T> {
        // Search through object styles and find one matching the given id
        if let Some(styles) = &self.get_styles() {
            // Note: Maybe more effecient to implement styles as a HashMap, 
            //       to make lookups faster  
            for style in styles {
                if let Some(style_id) = &style.get_id() {
                    if style_id == id {
                        return Some(style);
                    }
                } 
            }
        }
        // If we reach this point, no style was found 
        None
    }
}
