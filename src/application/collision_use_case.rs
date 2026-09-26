use crate::domain::collision::{CollisionSide, detect};
use crate::domain::position::PositionError;
use crate::domain::world::World;

pub fn resolve_collisions(world: &mut World) -> Result<(), PositionError> {
    world.player_mut().become_airborne();

    for i in 0..world.obstacles().len() {
        let obstacle = &world.obstacles()[i];
        let Some(collision) = detect(world.player(), obstacle) else {
            continue;
        };
        let bounds = obstacle.bounds();

        let player = world.player_mut();
        match collision.side() {
            CollisionSide::Top => player.land(bounds.top())?,
            CollisionSide::Bottom => player.bump_head(bounds.bottom())?,
            CollisionSide::Left => player.stop_left_of(bounds.left())?,
            CollisionSide::Right => player.stop_right_of(bounds.right())?,
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::color::Color;
    use crate::domain::life::Life;
    use crate::domain::obstacle::Obstacle;
    use crate::domain::player::Player;
    use crate::domain::position::Position;
    use crate::domain::size::Size;
    use crate::domain::state::State;
    use crate::domain::velocity::Velocity;

    fn player_at(x: f32, y: f32, velocity: f32) -> Player {
        Player {
            life: Life::Alive,
            size: Size::new(20.0, 20.0).unwrap(),
            velocity: Velocity::new(velocity),
            position: Position::new(x, y).unwrap(),
            state: State::Jumping,
            color: Color::Blue,
        }
    }

    // Obstacle spans x 100..200, y 100..150
    fn world_with(player: Player) -> World {
        let obstacle = Obstacle::new(
            Position::new(100.0, 100.0).unwrap(),
            Size::new(100.0, 50.0).unwrap(),
        );
        World::new(player, vec![obstacle], 1000.0)
    }

    #[test]
    fn landing_places_player_on_top_and_grounds_it() {
        let mut world = world_with(player_at(120.0, 85.0, 300.0));

        resolve_collisions(&mut world).unwrap();

        let player = world.player();
        assert_eq!(player.position.y(), 80.0);
        assert_eq!(player.velocity, Velocity::stationary());
        assert!(player.state == State::Grounded);
    }

    #[test]
    fn hitting_head_stops_upward_velocity_and_keeps_player_airborne() {
        let mut world = world_with(player_at(120.0, 145.0, -300.0));

        resolve_collisions(&mut world).unwrap();

        let player = world.player();
        assert_eq!(player.position.y(), 150.0);
        assert_eq!(player.velocity, Velocity::stationary());
        assert!(player.state == State::Jumping);
    }

    #[test]
    fn hitting_wall_from_left_pushes_player_out() {
        let mut world = world_with(player_at(85.0, 115.0, 0.0));

        resolve_collisions(&mut world).unwrap();

        assert_eq!(world.player().position.x(), 80.0);
    }

    #[test]
    fn hitting_wall_from_right_pushes_player_out() {
        let mut world = world_with(player_at(195.0, 115.0, 0.0));

        resolve_collisions(&mut world).unwrap();

        assert_eq!(world.player().position.x(), 200.0);
    }

    #[test]
    fn player_without_collision_is_airborne_and_unchanged() {
        let mut world = world_with(player_at(0.0, 0.0, 50.0));

        resolve_collisions(&mut world).unwrap();

        let player = world.player();
        assert_eq!(player.position.x(), 0.0);
        assert_eq!(player.position.y(), 0.0);
        assert_eq!(player.velocity, Velocity::new(50.0));
        assert!(player.state == State::Jumping);
    }
}
