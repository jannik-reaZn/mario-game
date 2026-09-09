pub struct Position(f32, f32);

impl Position {
    pub fn new(x: f32, y: f32) -> Result<Self, PositionError> {
        if x < 0.0 {
            return Err(PositionError::NegativeX);
        } else if y < 0.0 {
            return Err(PositionError::NegativeY);
        }
        Ok(Self(x, y))
    }

    /// Returns the x-coordinate
    pub fn get_x(&self) -> f32 {
        self.0
    }

    /// Returns the y-coordinate
    pub fn get_y(&self) -> f32 {
        self.1
    }

    /// Creates a new position with explicit x-coordinate
    pub fn with_x(&mut self, x: f32) -> Self {
        Self(x, self.1)
    }
    /// Creates a new position with explicit y-coordinate
    pub fn with_y(&self, y: f32) -> Self {
        Self(self.0, y)
    }

    /// Creates a new position with updated x-coordinate
    pub fn move_x(&mut self, amount: f32) -> Self {
        let new_x = (self.0 + amount).max(0.0);
        Self(new_x, self.1)
    }
}

#[derive(Debug)]
pub enum PositionError {
    NegativeX,
    NegativeY,
}
