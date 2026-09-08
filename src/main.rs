use macroquad::prelude::*;
mod domain;

use domain::constants::GAME_NAME;

#[macroquad::main("Mario 2D Game")]
async fn main() {
    loop {
        clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);

        draw_text(GAME_NAME, 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}
