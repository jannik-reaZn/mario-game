use crate::domain::bounds::Bounds;
use crate::domain::color::Color;
use crate::domain::position::{Position, PositionError};
use crate::domain::size::Size;
use crate::domain::state::State;
use crate::domain::velocity::Velocity;

pub const GRAVITY: f32 = 9.81 * 100.0;
pub const JUMP_STRENGTH: f32 = 500.0;

pub struct Player {
    pub size: Size,
    pub velocity: Velocity,
    pub position: Position,
    pub state: State,
    pub color: Color,
}

impl Player {
    pub fn bounds(&self) -> Bounds {
        Bounds::new(
            self.position.x(),
            self.position.x() + self.size.width(),
            self.position.y(),
            self.position.y() + self.size.height(),
        )
    }

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
        self.position = self.position.with_y(ground_y - self.size.height())?;
        self.velocity = Velocity::stationary();
        self.state = State::Grounded;

        Ok(())
    }

    pub fn become_airborne(&mut self) {
        self.state = State::Jumping;
    }

    pub fn bump_head(&mut self, ceiling_y: f32) -> Result<(), PositionError> {
        self.position = self.position.with_y(ceiling_y)?;
        self.velocity = Velocity::stationary();

        Ok(())
    }

    pub fn stop_left_of(&mut self, wall_x: f32) -> Result<(), PositionError> {
        self.position = self.position.with_x(wall_x - self.size.width())?;

        Ok(())
    }

    pub fn stop_right_of(&mut self, wall_x: f32) -> Result<(), PositionError> {
        self.position = self.position.with_x(wall_x)?;

        Ok(())
    }
}
