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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Velocity {
    value: f32,
}

impl Velocity {
    pub fn new(value: f32) -> Self {
        Self { value }
    }

    pub fn get_value(&self) -> f32 {
        self.value
    }
    pub fn accelerate(&self, acceleration: f32, dt: f32) -> Self {
        Self {
            value: self.value + acceleration * dt,
        }
    }

    pub fn stationary() -> Self {
        Self { value: 0.0 }
    }
}

pub struct Player {
    pub radius: f32,
    pub velocity: Velocity,
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
        self.velocity = self.velocity.accelerate(GRAVITY, dt);

        let y = self.position.get_y() + self.velocity.get_value() * dt;

        Position::new(self.position.get_x(), y)
    }
}
