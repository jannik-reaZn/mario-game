use macroquad::{
    time::get_frame_time,
    window::{screen_height, screen_width},
};

pub fn get_screen_width() -> f32 {
    screen_width()
}

pub fn get_screen_height() -> f32 {
    screen_height()
}

pub fn get_current_frame_time() -> f32 {
    get_frame_time()
}
