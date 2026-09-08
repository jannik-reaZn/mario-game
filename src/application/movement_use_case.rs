use crate::domain::movement::PlayerMovement;
use crate::domain::position::{Position, PositionError};

#[allow(dead_code)]
pub fn movement_use_case(movement: PlayerMovement) -> Result<Position, PositionError> {
    match movement {
        PlayerMovement::Right => Position::new(0.0, 0.0),
        PlayerMovement::Left => Position::new(0.0, 0.0),
        PlayerMovement::Jump => Position::new(0.0, 0.0),
    }
}
