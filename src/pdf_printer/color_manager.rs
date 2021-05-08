use crate::idml_parser::graphic_parser::{Color as IdmlColor, ColorSpace, Swatch};
use crate::idml_parser::IDMLResources;
use derive_getters::Getters;

#[derive(Debug, Clone, Copy, Getters)]
pub struct Cmyk {
    c: f32,
    m: f32,
    y: f32,
    k: f32,
}

#[derive(Debug, Clone, Copy, Getters)]
pub struct Rgb {
    r: f32,
    g: f32,
    b: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Cmyk(Cmyk),
    Rgb(Rgb),
    None,
}

#[derive(Debug)]
pub enum ColorError {
    ColorNotImplemented,
    NoColorMatch,
    MultiColorMatch,
}

impl std::fmt::Display for ColorError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let text = match &self {
            ColorError::ColorNotImplemented => "Color not implemented",
            ColorError::NoColorMatch => "No colors match id provided",
            ColorError::MultiColorMatch => "Multiple colors match the id provided",
        };
        write!(f, "{}", text)
    }
}

impl std::error::Error for ColorError {}

pub fn color_from_id(idml_resources: &IDMLResources, id: &String) -> Result<Color, ColorError> {
    idml_resources.color_from_id(id)
}

impl IDMLResources {
    pub fn color_from_id(&self, id: &String) -> Result<Color, ColorError> {
        // List to search
        let mut matches = vec![];

        // Append colors that match id
        matches.append(
            &mut self
                .graphic()
                .colors()
                .into_iter()
                .filter(|color| color.id() == id)
                .map(|color| color as &dyn ToPDFColor)
                .collect::<Vec<&dyn ToPDFColor>>(),
        );

        // Append swatches that match id
        matches.append(
            &mut self
                .graphic()
                .swatches()
                .into_iter()
                .filter(|swatch| swatch.id() == id)
                .map(|swatch| swatch as &dyn ToPDFColor)
                .collect::<Vec<&dyn ToPDFColor>>(),
        );

        // Return color if found
        match matches[..] {
            [] => Err(ColorError::NoColorMatch),
            [color] => color.to_pdf_color(),
            [_, ..] => Err(ColorError::MultiColorMatch),
        }
    }
}

pub trait ToPDFColor {
    fn to_pdf_color(&self) -> Result<Color, ColorError>;
}

impl ToPDFColor for IdmlColor {
    fn to_pdf_color(&self) -> Result<Color, ColorError> {
        // println!("Color: {:#?}", &self.color_value());

        match (&self.space(), &self.color_value()) {
            (Some(ColorSpace::CMYK), Some(value)) => {
                // Normalise values
                let value = value.iter().map(|v| v / 100_f64).collect::<Vec<f64>>();

                Ok(Color::Cmyk(Cmyk {
                    c: value[0] as f32,
                    m: value[1] as f32,
                    y: value[2] as f32,
                    k: value[3] as f32,
                }))
            }
            (Some(ColorSpace::RGB), Some(value)) => {
                // Normalise values
                let value = value.iter().map(|v| v / 255_f64).collect::<Vec<f64>>();

                Ok(Color::Rgb(Rgb {
                    r: value[0] as f32,
                    g: value[1] as f32,
                    b: value[2] as f32,
                }))
            }
            _ => Err(ColorError::ColorNotImplemented),
        }
    }
}

impl ToPDFColor for Swatch {
    fn to_pdf_color(&self) -> Result<Color, ColorError> {
        // Only the "Swatch/None" should ever be created in IDML, so just default to not implemented yet
        Err(ColorError::ColorNotImplemented)
    }
}
