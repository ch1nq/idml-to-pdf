use crate::idml_parser::graphic_parser::{Color as IdmlColor, ColorSpace};
use crate::idml_parser::IDMLResources;
use printpdf::{Cmyk, Color as PdfColor, Rgb, SpotColor};

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

pub fn color_from_id(
    idml_resources: &IDMLResources,
    id: &String,
) -> Result<Option<PdfColor>, ColorError> {
    idml_resources.color_from_id(id)
}

impl IDMLResources {
    pub fn color_from_id(&self, id: &String) -> Result<Option<PdfColor>, ColorError> {
        let color_lookup = &self
            .graphic()
            .colors()
            .into_iter()
            .filter(|color| {
                if let Some(color_id) = color.id() {
                    color_id == id
                } else {
                    false
                }
            })
            .collect::<Vec<&IdmlColor>>();

        match color_lookup.len() {
            0 => {
                // TOOD: Load swatches so it doenst fail when a swatch is used.
                //       Just return None for now

                // FIXME: Temporary fix until swatches get implemented
                if id == "Swatch/None" {
                    return Err(ColorError::ColorNotImplemented);
                }

                Err(ColorError::NoColorMatch)
            }
            1 => Ok(Some(color_lookup[0].to_pdf_color())),
            _ => Err(ColorError::MultiColorMatch),
        }
    }
}

impl IdmlColor {
    pub fn to_pdf_color(&self) -> PdfColor {
        // println!("Color: {:#?}", &self.color_value());

        match (&self.space(), &self.color_value()) {
            (Some(ColorSpace::CMYK), Some(value)) => {
                // Normalise values
                let value = value.iter().map(|v| v / 100_f64).collect::<Vec<f64>>();

                PdfColor::Cmyk(Cmyk::new(value[0], value[1], value[2], value[3], None))
            }
            (Some(ColorSpace::RGB), Some(value)) => {
                // Normalise values
                let value = value.iter().map(|v| v / 255_f64).collect::<Vec<f64>>();

                PdfColor::Rgb(Rgb::new(value[0], value[1], value[2], None))
            }
            (space, _) => {
                println!("Color of type '{:#?}' is not implemented yet", space);
                // Default color is 100% magenta
                PdfColor::Cmyk(Cmyk::new(0.0, 1.0, 0.0, 0.0, None))
            }
        }
    }
}
