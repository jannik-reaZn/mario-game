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
    pub fn move_right(&self, amount: f32) -> Result<Position, PositionError> {
        self.position.move_x(amount)
    }

    pub fn move_left(&self, amount: f32) -> Result<Position, PositionError> {
        self.position.move_x(-amount)
    }

    pub fn jump(&mut self) {
        self.velocity = Velocity::new(-JUMP_STRENGTH);
    }

    pub fn apply_gravity(&mut self, dt: f32) {
        self.velocity = self.velocity.accelerate(GRAVITY, dt)
    }

    pub fn calculate_next_position(&self, dt: f32) -> Result<Position, PositionError> {
        let y = self.position.get_y() + self.velocity.value() * dt;

        Position::new(self.position.get_x(), y)
    }

    pub fn update_position(&mut self, dt: f32) -> Result<(), PositionError> {
        self.position = self.calculate_next_position(dt)?;
        Ok(())
    }

    pub fn land(&mut self, ground_y: f32) -> Result<(), PositionError> {
        self.position = self.position.with_y(ground_y - self.radius.value())?;
        self.velocity = Velocity::stationary();
        Ok(())
    }
}
