use crate::presentation::constants::GAME_NAME;
use macroquad::{
    time::get_frame_time,
    window::{Conf, screen_height, screen_width},
};

/// Window settings, handed to `#[macroquad::main(...)]`.
///
/// The macro cannot take a constant directly (it only accepts a string
/// literal or the name of a function returning `Conf`), so this function is
/// the bridge that lets the title come from `GAME_NAME`.
pub fn window_conf() -> Conf {
    Conf {
        window_title: GAME_NAME.to_owned(),
        ..Default::default()
    }
}

pub fn get_screen_width() -> f32 {
    screen_width()
}

pub fn get_screen_height() -> f32 {
    screen_height()
}

pub fn get_current_frame_time() -> f32 {
    get_frame_time()
}
