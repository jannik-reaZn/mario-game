use crate::domain::position::{Position, PositionError};
use macroquad::color::Color;

pub const GRAVITY: f32 = 9.81 * 100.0;
pub const JUMP_STRENGTH: f32 = 500.0;

pub enum PlayerState {
    Idle,
    Jumping,
    Falling,
}

pub enum PlayerMovement {
    Right,
    Left,
    Jump,
}

pub struct Player {
    pub radius: f32,
    pub velocity: f32,
    pub position: Position,
    pub color: Color, // TODO this needs to be String instead of Color
}

/// Calculates the next y position after each frame.
/// The players's velocity starts negative (upward) and continously increases.
/// Eventually vy reaches zero at the top of the jump.
/// The velocity becomes positive (downward).
///
/// y: The position on y-axis
/// dt: Delta time
impl Player {
    pub fn calculate_y_position(&mut self, dt: f32) -> Result<Position, PositionError> {
        self.velocity += GRAVITY * dt;

        let y = self.position.get_y() + self.velocity * dt;

        Position::new(self.position.get_x(), y)
    }

    pub fn update_position(player: &mut Player, dt: f32) -> Result<(), PositionError> {
        player.position = player.calculate_y_position(dt)?;
        Ok(())
    }
}
