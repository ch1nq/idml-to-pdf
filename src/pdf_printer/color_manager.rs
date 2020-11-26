use printpdf::{Color, Rgb, Cmyk};

pub fn color_from_id(id: &String) -> Color {

    // Test
    Color::Cmyk(Cmyk::new(0.0, 1.0, 0.0, 0.0, None))
}
