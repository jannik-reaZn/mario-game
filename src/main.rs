use macroquad::prelude::*;
mod application;
mod domain;
mod presentation;

use application::collision_use_case::resolve_collisions;
use application::movement_use_case::move_player;
use domain::constants::GAME_NAME;
use domain::obstacle::{Obstacle, ObstacleType};
use domain::player::Player;
use domain::position::Position;
use domain::size::Size;
use domain::velocity::Velocity;
use domain::world::World;
use presentation::constants::{BACKGROUND_COLOR, GROUND_Y, PLAYER_COLOR};
use presentation::window::{get_current_frame_time, get_screen_height, get_screen_width};

use crate::domain::movement::PlayerMovement;
use crate::domain::state::State;

#[macroquad::main("Mario 2D Game")]
async fn main() {
    let player = Player {
        size: Size::new(32.0, 32.0).expect("Ok"),
        velocity: Velocity::new(0.0),
        position: Position::new(get_screen_width() / 2.0, get_screen_height() / 2.0).expect("Ok"),
        state: State::Grounded,
        color: PLAYER_COLOR,
    };

    let obstacles = vec![
        Obstacle::new(
            ObstacleType::Solid,
            Position::new(0.0, GROUND_Y).expect("Ok"),
            Size::new(2000.0, 50.0).expect("Ok"),
        ),
        Obstacle::new(
            ObstacleType::Solid,
            Position::new(300.0, GROUND_Y - 100.0).expect("Ok"),
            Size::new(150.0, 30.0).expect("Ok"),
        ),
        Obstacle::new(
            ObstacleType::Solid,
            Position::new(600.0, GROUND_Y - 60.0).expect("Ok"),
            Size::new(60.0, 60.0).expect("Ok"),
        ),
    ];

    let mut world = World::new(player, obstacles);

    loop {
        clear_background(BACKGROUND_COLOR);
        draw_text(GAME_NAME, 20.0, 20.0, 30.0, DARKGRAY);

        // Logic
        let delta_time = get_current_frame_time();

        // Movement Right
        if is_key_down(KeyCode::Right) {
            move_player(world.player_mut(), PlayerMovement::Right, delta_time)
                .expect("Player movement should produce a valid position");
        }

        // Movement Left
        if is_key_down(KeyCode::Left) {
            move_player(world.player_mut(), PlayerMovement::Left, delta_time)
                .expect("Player movement should produce a valid position");
        }

        // Movement Jump
        if is_key_pressed(KeyCode::Space) {
            move_player(world.player_mut(), PlayerMovement::Jump, delta_time)
                .expect("Jump should always be valid");
        }

        // Gravity + vertical movement
        world.player_mut().apply_gravity(delta_time);
        world
            .player_mut()
            .update_position(delta_time)
            .expect("Player physics should produce a valid position");

        // Collisions with obstacles
        resolve_collisions(&mut world)
            .expect("Collision resolution should produce a valid position");

        // Make sure that the player does not run outside the screen
        let player = world.player_mut();
        let x = clamp(
            player.position.x(),
            0.0,
            screen_width() - player.size.width(),
        );
        player.position = player.position.with_x(x).unwrap();

        // Render
        for obstacle in world.obstacles() {
            let bounds = obstacle.bounds();
            draw_rectangle(
                bounds.left(),
                bounds.top(),
                bounds.right() - bounds.left(),
                bounds.bottom() - bounds.top(),
                BLACK,
            );
        }

        let player = world.player();
        draw_rectangle(
            player.position.x(),
            player.position.y(),
            player.size.width(),
            player.size.height(),
            player.color,
        );

        next_frame().await
    }
}
