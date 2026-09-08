use macroquad::prelude::*;
mod domain;
mod presentation;

use domain::constants::GAME_NAME;
use presentation::constants::BACKGROUND_COLOR;

#[macroquad::main("Mario 2D Game")]
async fn main() {
    loop {
        clear_background(BACKGROUND_COLOR);

        // Title
        draw_text(GAME_NAME, 20.0, 20.0, 30.0, DARKGRAY);

        // Boxes
        draw_line(0.0, 800.0, 1000.0, 800.0, 15.0, BLACK);
        // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);

        // Render
        next_frame().await
    }
}
