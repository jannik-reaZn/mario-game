use crate::domain::movement::PlayerMovement;
use crate::domain::player::Player;
use crate::domain::position::PositionError;

use crate::presentation::constants::MOVEMENT_SPEED;

pub fn move_player(
    player: &mut Player,
    movement: PlayerMovement,
    dt: f32,
) -> Result<(), PositionError> {
    match movement {
        PlayerMovement::Right => {
            player.position = player.move_right(MOVEMENT_SPEED * dt)?;
        }

        PlayerMovement::Left => {
            player.position = player.move_left(MOVEMENT_SPEED * dt)?;
        }

        PlayerMovement::Jump => {
            player.jump();
        }
    }

    Ok(())
}
