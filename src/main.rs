mod application;
mod domain;
mod presentation;
mod world_factory;

use application::check_player_death_use_case::check_player_death;
use application::collision_use_case::resolve_collisions;
use application::movement_use_case::move_player;
use macroquad::color::DARKGRAY;
use macroquad::math::clamp;
use macroquad::shapes::draw_rectangle;
use macroquad::text::draw_text;
use macroquad::window::{clear_background, next_frame};
use presentation::constants::{BACKGROUND_COLOR, GAME_NAME, OBSTACLE_COLOR};
use presentation::game_over::draw_game_over;
use presentation::input::read_player_movements;
use presentation::window::{
    get_current_frame_time, get_screen_height, get_screen_width, window_conf,
};
use world_factory::build_world;

#[macroquad::main(window_conf)]
async fn main() {
    let mut world = build_world(get_screen_width(), get_screen_height());

    loop {
        clear_background(BACKGROUND_COLOR.into());
        let dt = get_current_frame_time();
        draw_text(GAME_NAME, 20.0, 20.0, 30.0, DARKGRAY);

        // A dead player freezes the world; the scene is still drawn below.
        if world.player().is_alive() {
            // Input -> movement
            for movement in read_player_movements() {
                move_player(world.player_mut(), movement, dt)
                    .expect("Player movement should produce a valid position");
            }

            // Gravity + vertical movement
            world.player_mut().apply_gravity(dt);
            world
                .player_mut()
                .update_position(dt)
                .expect("Player physics should produce a valid position");

            // Collisions with obstacles
            resolve_collisions(&mut world)
                .expect("Collision resolution should produce a valid position");

            // Falling out of the level (after collisions, so a landing wins)
            check_player_death(&mut world);

            // Make sure that the player does not run outside the screen
            let player = world.player_mut();
            let x = clamp(
                player.position.x(),
                0.0,
                get_screen_width() - player.size.width(),
            );
            player.position = player.position.with_x(x).unwrap();
        }

        // Render
        for obstacle in world.obstacles() {
            let bounds = obstacle.bounds();
            draw_rectangle(
                bounds.left(),
                bounds.top(),
                bounds.right() - bounds.left(),
                bounds.bottom() - bounds.top(),
                OBSTACLE_COLOR.into(),
            );
        }

        let player = world.player();
        draw_rectangle(
            player.position.x(),
            player.position.y(),
            player.size.width(),
            player.size.height(),
            player.color.into(),
        );

        // Game over pop-up; a click on "Retry" replaces the whole world
        if world.player().is_dead() && draw_game_over() {
            world = build_world(get_screen_width(), get_screen_height());
        }

        next_frame().await
    }
}
