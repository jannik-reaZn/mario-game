use crate::domain::position::{Position, PositionError};
use crate::domain::radius::Radius;
use crate::domain::state::State;
use crate::domain::velocity::Velocity;
use macroquad::color::Color;

pub const GRAVITY: f32 = 9.81 * 100.0;
pub const JUMP_STRENGTH: f32 = 500.0;

pub struct Player {
    pub radius: Radius,
    pub velocity: Velocity,
    pub position: Position,
    pub state: State,
    pub color: Color, // TODO this needs to be String instead of Color
}

impl Player {
    pub fn jump(&mut self) {
        self.velocity = Velocity::new(-JUMP_STRENGTH);
        self.state = State::Jumping;
    }

    pub fn apply_gravity(&mut self, dt: f32) {
        self.velocity = self.velocity.accelerate(GRAVITY, dt);
    }

    pub fn update_position(&mut self, dt: f32) -> Result<(), PositionError> {
        let y = self.position.y() + self.velocity.value() * dt;

        self.position = Position::new(self.position.x(), y)?;

        Ok(())
    }

    pub fn land(&mut self, ground_y: f32) -> Result<(), PositionError> {
        self.position = self.position.with_y(ground_y - self.radius.value())?;
        self.velocity = Velocity::stationary();
        self.state = State::Grounded;

        Ok(())
    }
}
