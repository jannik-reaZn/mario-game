use crate::domain::color::Color;

// COLOR
pub const BACKGROUND_COLOR: Color = Color::White;
pub const PLAYER_COLOR: Color = Color::Blue;
pub const OBSTACLE_COLOR: Color = Color::Black;

// WINDOW
pub const GAME_NAME: &str = "Mario 2D Game";
pub const GROUND_Y: f32 = 800.0;
/// Below this `y` the player has fallen out of the level.
pub const WORLD_BOTTOM: f32 = GROUND_Y + 100.0;
