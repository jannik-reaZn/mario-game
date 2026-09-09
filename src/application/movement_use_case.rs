use crate::domain::movement::PlayerMovement;
use crate::domain::player::Player;
use crate::domain::position::PositionError;

use crate::domain::state::State;
use crate::presentation::constants::MOVEMENT_SPEED;

pub fn move_player(
    player: &mut Player,
    movement: PlayerMovement,
    dt: f32,
) -> Result<(), PositionError> {
    match movement {
        PlayerMovement::Right => {
            player.position = player.position.move_x(MOVEMENT_SPEED * dt)?;
        }

        PlayerMovement::Left => {
            player.position = player.position.move_x(-MOVEMENT_SPEED * dt)?;
        }

        PlayerMovement::Jump => {
            if player.state == State::Grounded {
                player.jump();
            }
        }
    }

    Ok(())
}
