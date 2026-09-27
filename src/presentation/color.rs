use crate::domain::color::Color;
use macroquad::color::{BLACK, BLUE, Color as MqColor, DARKGRAY, WHITE};

/// Translates the domain's color into the one Macroquad can draw.
///
/// The `match` is exhaustive: adding a variant to the domain `Color`
/// will not compile until it is mapped here.
impl From<Color> for MqColor {
    fn from(color: Color) -> Self {
        match color {
            Color::Blue => BLUE,
            Color::White => WHITE,
            Color::Black => BLACK,
            Color::DARKGREY => DARKGRAY,
        }
    }
}
