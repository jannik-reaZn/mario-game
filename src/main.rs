use macroquad::prelude::*;
mod domain;
mod presentation;

use domain::constants::GAME_NAME;
use presentation::constants::{BACKGROUND_COLOR, MOVEMENT_SPEED, PLAYER_COLOR};

use crate::presentation::window::{get_current_frame_time, get_screen_height, get_screen_width};

#[macroquad::main("Mario 2D Game")]
async fn main() {
    let mut x = get_screen_width() / 2.0;
    let mut y = get_screen_height() / 2.0;

    loop {
        clear_background(BACKGROUND_COLOR);
        draw_text(GAME_NAME, 20.0, 20.0, 30.0, DARKGRAY);

        // Logic
        let delta_time = get_current_frame_time();

        if is_key_down(KeyCode::Right) {
            x += MOVEMENT_SPEED + delta_time;
        }
        if is_key_down(KeyCode::Left) {
            x -= MOVEMENT_SPEED + delta_time;
        }
        if is_key_down(KeyCode::Down) {
            y += MOVEMENT_SPEED + delta_time;
        }
        if is_key_down(KeyCode::Up) {
            y -= MOVEMENT_SPEED + delta_time;
        }

        // NOTE: 8.0 is hardcoded currently. It should be half of the players model!
        x = clamp(x, 0.0 + 8.0, screen_width());
        y = clamp(y, 0.0 + 8.0, screen_height());

        // Boxes
        draw_line(0.0, 800.0, 1000.0, 800.0, 15.0, BLACK);
        draw_circle(x, y, 16.0, PLAYER_COLOR);

        // Render
        next_frame().await
    }
}
