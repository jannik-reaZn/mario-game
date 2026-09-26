use crate::domain::world::World;

pub fn check_player_death(world: &mut World) {
    if world.player().has_fallen_below(world.bottom()) {
        world.player_mut().die();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::color::Color;
    use crate::domain::life::Life;
    use crate::domain::player::Player;
    use crate::domain::position::Position;
    use crate::domain::size::Size;
    use crate::domain::state::State;
    use crate::domain::velocity::Velocity;

    const BOTTOM: f32 = 1000.0;

    fn world_with_player_at(x: f32, y: f32) -> World {
        let player = Player {
            life: Life::Alive,
            size: Size::new(20.0, 20.0).unwrap(),
            velocity: Velocity::new(400.0),
            position: Position::new(x, y).unwrap(),
            state: State::Jumping,
            color: Color::Blue,
        };
        World::new(player, vec![], BOTTOM)
    }

    #[test]
    fn player_below_the_bottom_dies_and_stops_moving() {
        let mut world = world_with_player_at(100.0, BOTTOM + 1.0);

        check_player_death(&mut world);

        assert!(world.player().is_dead());
        assert_eq!(world.player().velocity, Velocity::stationary());
    }

    #[test]
    fn player_still_inside_the_level_stays_alive() {
        let mut world = world_with_player_at(100.0, 500.0);

        check_player_death(&mut world);

        assert!(world.player().is_alive());
        assert_eq!(world.player().velocity, Velocity::new(400.0));
    }

    #[test]
    fn player_exactly_on_the_limit_stays_alive() {
        let mut world = world_with_player_at(100.0, BOTTOM);

        check_player_death(&mut world);

        assert!(world.player().is_alive());
    }

    #[test]
    fn player_at_the_left_screen_edge_stays_alive() {
        let mut world = world_with_player_at(0.0, 500.0);

        check_player_death(&mut world);

        assert!(world.player().is_alive());
    }
}
