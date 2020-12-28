pub mod character_style;
pub mod paragraph_style;
pub mod object_style;

use std::borrow::Cow;

pub trait Style {
    fn get_id(&self) -> &Option<String>;
    fn get_parent_id(&self) -> &Option<String>;
    fn combine_with_parent(&self, parent: &Self) -> Self;

    /// Use this to override parent value with child value if it is specified  
    fn choose<U> (&self, child:Option<U>, parent:Option<U>) -> Option<U> {
        if child.is_some() {
            child
        } else {
            parent
        }
    }
}

/// Defines shared behaviour for style groups
pub trait StyleGroup<T: Style + Clone + std::fmt::Debug> {
    /// Returns a list of all the styles in the group 
    fn get_styles(&self) -> &Option<Vec<T>>;

    /// Get a style based given an IDML style id 
    fn style_from_id(&self, id: &String) -> Option<T> {
        // Search through object styles and find one matching the given id
        if let Some(styles) = &self.get_styles() {
            // Note: Maybe more effecient to implement styles as a HashMap, 
            //       to make lookups faster  
            for style in styles {
                if let Some(style_id) = &style.get_id() {
                    if id == style_id || (id == "$ID/[No paragraph style]" && style_id == "ParagraphStyle/$ID/[No paragraph style]") {
                        return match self.with_parent_properties(style){
                            Cow::Borrowed(b) => Some(b.to_owned()),
                            Cow::Owned(o) => Some(o)
                        }
                    }
                } 
            }
        }

        // If we reach this point, no style was found 
        None
    }
    
    /// Returns a style also containing its parents attributes  
    fn with_parent_properties<'a>(&self, style: &'a T) -> Cow<'a, T> {
        if let Some(parent_id) = style.get_parent_id() {
            if let Some(parent_style) = &self.style_from_id(parent_id) {
                return Cow::Owned(style.combine_with_parent(parent_style));
            }
        }

        Cow::Borrowed(style)
    }
}
