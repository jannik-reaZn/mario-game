use crate::domain::life::Life;
use crate::domain::obstacle::Obstacle;
use crate::domain::player::Player;
use crate::domain::position::Position;
use crate::domain::size::Size;
use crate::domain::state::State;
use crate::domain::velocity::Velocity;
use crate::domain::world::World;
use crate::presentation::constants::{GROUND_Y, PLAYER_COLOR, WORLD_BOTTOM};

/// Creates a fresh game: the level layout with the player at the center of
/// the screen. Used for the first start and for every retry.
pub fn build_world(screen_width: f32, screen_height: f32) -> World {
    let player = Player {
        life: Life::Alive,
        size: Size::new(32.0, 32.0).expect("player size is valid"),
        velocity: Velocity::stationary(),
        position: Position::new(screen_width / 2.0, screen_height / 2.0)
            .expect("screen center is a valid position"),
        state: State::Grounded,
        color: PLAYER_COLOR,
    };

    let obstacles = vec![
        // Ground, split by a pit between x = 1000 and x = 1200
        obstacle(1200.0, GROUND_Y, 2000.0, 50.0),
        obstacle(0.0, GROUND_Y, 1000.0, 50.0),
        // Platform and block
        obstacle(300.0, GROUND_Y - 100.0, 150.0, 30.0),
        obstacle(600.0, GROUND_Y - 60.0, 60.0, 60.0),
    ];

    World::new(player, obstacles, WORLD_BOTTOM)
}

fn obstacle(x: f32, y: f32, width: f32, height: f32) -> Obstacle {
    Obstacle::new(
        Position::new(x, y).expect("obstacle position is valid"),
        Size::new(width, height).expect("obstacle size is valid"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_starts_alive_and_grounded_at_the_screen_center() {
        let world = build_world(1000.0, 600.0);

        let player = world.player();
        assert!(player.is_alive());
        assert!(player.state == State::Grounded);
        assert_eq!(player.position.x(), 500.0);
        assert_eq!(player.position.y(), 300.0);
    }

    #[test]
    fn level_has_its_obstacles_and_bottom() {
        let world = build_world(1000.0, 600.0);

        assert_eq!(world.obstacles().len(), 4);
        assert_eq!(world.bottom(), WORLD_BOTTOM);
    }

    #[test]
    fn every_call_builds_an_independent_world() {
        let mut first = build_world(1000.0, 600.0);
        first.player_mut().die();

        let second = build_world(1000.0, 600.0);

        assert!(second.player().is_alive());
    }
}
