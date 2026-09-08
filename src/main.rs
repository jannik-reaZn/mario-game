use macroquad::prelude::*;
mod application;
mod domain;
mod presentation;

use domain::constants::GAME_NAME;
use domain::player::{JUMP_STRENGTH, Player};
use presentation::constants::{BACKGROUND_COLOR, GROUND_Y, MOVEMENT_SPEED, PLAYER_COLOR};
use presentation::window::{get_current_frame_time, get_screen_height, get_screen_width};

#[macroquad::main("Mario 2D Game")]
async fn main() {
    let mut player = Player {
        radius: 16.0,
        velocity: 0.0,
        position: (get_screen_width() / 2.0, get_screen_height() / 2.0),
        color: PLAYER_COLOR,
    };

    loop {
        clear_background(BACKGROUND_COLOR);
        draw_text(GAME_NAME, 20.0, 20.0, 30.0, DARKGRAY);

        // Logic
        let delta_time = get_current_frame_time();

        // Movement Right
        if is_key_down(KeyCode::Right) {
            player.position.0 += MOVEMENT_SPEED * delta_time;
        }

        // Movement Left
        if is_key_down(KeyCode::Left) {
            player.position.0 -= MOVEMENT_SPEED * delta_time;
        }

        // Movement Jump
        if is_key_pressed(KeyCode::Space) {
            player.velocity = -JUMP_STRENGTH;
        }
        // Gravity + vertical movement
        player.calculate_y_position(delta_time);

        // Ground Collision
        if player.position.1 >= GROUND_Y - player.radius {
            player.position.1 = GROUND_Y - player.radius;
            player.velocity = 0.0;
        }

        // Make sure that the player does not run outside the screen
        player.position.0 = clamp(player.position.0, 0.0 + player.radius / 2.0, screen_width());

        // Boxes
        draw_line(0.0, GROUND_Y, 1000.0, GROUND_Y, 15.0, BLACK);
        draw_circle(
            player.position.0,
            player.position.1,
            player.radius,
            player.color,
        );

        // Render
        next_frame().await
    }
}
