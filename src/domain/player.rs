use crate::domain::bounds::Bounds;
use crate::domain::color::Color;
use crate::domain::life::Life;
use crate::domain::position::{Position, PositionError};
use crate::domain::size::Size;
use crate::domain::state::State;
use crate::domain::velocity::Velocity;

pub const GRAVITY: f32 = 9.81 * 100.0;
pub const JUMP_STRENGTH: f32 = 500.0;
pub const MOVEMENT_SPEED: f32 = 300.0;

pub struct Player {
    pub life: Life,
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

    /// Moves the player horizontally, negative values walk left.
    ///
    /// The world starts at `x = 0`, so walking left stops there instead of
    /// producing an invalid (negative) position.
    pub fn walk(&mut self, amount: f32) -> Result<(), PositionError> {
        let x = (self.position.x() + amount).max(0.0);
        self.position = self.position.with_x(x)?;

        Ok(())
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

    /// True once the player has dropped completely below `limit_y`.
    ///
    /// `y` grows downwards, so "below" means a larger `y`. Touching the limit
    /// is not enough, the top edge of the player has to pass it.
    pub fn has_fallen_below(&self, limit_y: f32) -> bool {
        self.bounds().top() > limit_y
    }

    pub fn is_alive(&self) -> bool {
        self.life == Life::Alive
    }

    pub fn is_dead(&self) -> bool {
        self.life == Life::Dead
    }

    pub fn die(&mut self) {
        self.life = Life::Dead;
        self.velocity = Velocity::stationary();
    }
}
