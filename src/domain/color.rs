/// The colors the game itself knows about.
///
/// This is deliberately *not* a Macroquad type. It is the list of colors the
/// system accepts; how each one ends up on screen is decided by the
/// presentation layer.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    Blue,
    White,
    Black,
    DARKGREY,
}
