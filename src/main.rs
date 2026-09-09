use macroquad::prelude::*;
mod application;
mod domain;
mod presentation;

use application::movement_use_case::move_player;
use domain::constants::GAME_NAME;
use domain::player::Player;
use domain::position::Position;
use domain::radius::Radius;
use domain::velocity::Velocity;
use presentation::constants::{BACKGROUND_COLOR, GROUND_Y, PLAYER_COLOR};
use presentation::window::{get_current_frame_time, get_screen_height, get_screen_width};

use crate::domain::movement::PlayerMovement;
use crate::domain::state::State;

#[macroquad::main("Mario 2D Game")]
async fn main() {
    let mut player = Player {
        radius: Radius::new(16.0).expect("Ok"),
        velocity: Velocity::new(0.0),
        position: Position::new(get_screen_width() / 2.0, get_screen_height() / 2.0).expect("Ok"),
        state: State::Grounded,
        color: PLAYER_COLOR,
    };

    loop {
        clear_background(BACKGROUND_COLOR);
        draw_text(GAME_NAME, 20.0, 20.0, 30.0, DARKGRAY);

        // Logic
        let delta_time = get_current_frame_time();

        // Movement Right
        if is_key_down(KeyCode::Right) {
            move_player(&mut player, PlayerMovement::Right, delta_time)
                .expect("Player movement should produce a valid position");
        }

        // Movement Left
        if is_key_down(KeyCode::Left) {
            move_player(&mut player, PlayerMovement::Left, delta_time)
                .expect("Player movement should produce a valid position");
        }

        // Movement Jump
        if is_key_pressed(KeyCode::Space) {
            move_player(&mut player, PlayerMovement::Jump, delta_time)
                .expect("Jump should always be valid");
        }
        // Gravity + vertical movement
        player.apply_gravity(delta_time);
        player
            .update_position(delta_time)
            .expect("Player physics should produce a valid position");

        // Ground Collision
        if player.position.y() >= GROUND_Y - player.radius.value() {
            player
                .land(GROUND_Y)
                .expect("Ground position should always be valid");
        }

        // Make sure that the player does not run outside the screen
        let x = clamp(
            player.position.x(),
            player.radius.value() / 2.0,
            screen_width(),
        );
        player.position = player.position.with_x(x).unwrap();

        // Boxes
        draw_line(0.0, GROUND_Y, 1000.0, GROUND_Y, 15.0, BLACK);
        draw_circle(
            player.position.x(),
            player.position.y(),
            player.radius.value(),
            player.color,
        );

        // Render
        next_frame().await
    }
}
