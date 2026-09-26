use crate::domain::movement::PlayerMovement;
use crate::domain::player::{MOVEMENT_SPEED, Player};
use crate::domain::position::PositionError;
use crate::domain::state::State;

pub fn move_player(
    player: &mut Player,
    movement: PlayerMovement,
    dt: f32,
) -> Result<(), PositionError> {
    match movement {
        PlayerMovement::Right => player.walk(MOVEMENT_SPEED * dt)?,

        PlayerMovement::Left => player.walk(-MOVEMENT_SPEED * dt)?,

        PlayerMovement::Jump => {
            if player.state == State::Grounded {
                player.jump();
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::color::Color;
    use crate::domain::life::Life;
    use crate::domain::position::Position;
    use crate::domain::size::Size;
    use crate::domain::velocity::Velocity;

    fn player_at(x: f32) -> Player {
        Player {
            life: Life::Alive,
            size: Size::new(20.0, 20.0).unwrap(),
            velocity: Velocity::stationary(),
            position: Position::new(x, 100.0).unwrap(),
            state: State::Grounded,
            color: Color::Blue,
        }
    }

    #[test]
    fn moving_right_increases_x() {
        let mut player = player_at(100.0);

        move_player(&mut player, PlayerMovement::Right, 0.5).unwrap();

        assert_eq!(player.position.x(), 100.0 + MOVEMENT_SPEED * 0.5);
    }

    #[test]
    fn moving_left_decreases_x() {
        let mut player = player_at(100.0);

        move_player(&mut player, PlayerMovement::Left, 0.1).unwrap();

        assert_eq!(player.position.x(), 100.0 - MOVEMENT_SPEED * 0.1);
    }

    #[test]
    fn moving_left_at_the_left_edge_stays_at_the_edge() {
        let mut player = player_at(0.0);

        let result = move_player(&mut player, PlayerMovement::Left, 0.1);

        assert!(result.is_ok());
        assert_eq!(player.position.x(), 0.0);
    }

    #[test]
    fn moving_left_past_the_left_edge_stops_at_the_edge() {
        let mut player = player_at(1.0);

        move_player(&mut player, PlayerMovement::Left, 0.1).unwrap();

        assert_eq!(player.position.x(), 0.0);
    }
}
