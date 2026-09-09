use crate::domain::position::{Position, PositionError};
use crate::domain::radius::Radius;
use crate::domain::velocity::Velocity;
use macroquad::color::Color;

pub const GRAVITY: f32 = 9.81 * 100.0;
pub const JUMP_STRENGTH: f32 = 500.0;

pub struct Player {
    pub radius: Radius,
    pub velocity: Velocity,
    pub position: Position,
    pub color: Color, // TODO this needs to be String instead of Color
}

/// Calculates the next y position after each frame.
/// The players's velocity starts negative (upward) and continously increases.
/// Eventually vy reaches zero at the top of the jump.
/// The velocity becomes positive (downward).
impl Player {
    pub fn calculate_y_position(&mut self, dt: f32) -> Result<Position, PositionError> {
        self.velocity = self.velocity.accelerate(GRAVITY, dt);

        let y = self.position.get_y() + self.velocity.value() * dt;

        Position::new(self.position.get_x(), y)
    }
}
