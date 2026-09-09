pub struct Position {
    x: f32,
    y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Result<Self, PositionError> {
        if x < 0.0 {
            return Err(PositionError::NegativeX);
        } else if y < 0.0 {
            return Err(PositionError::NegativeY);
        }
        Ok(Self { x, y })
    }

    /// Returns the x-coordinate
    pub fn get_x(&self) -> f32 {
        self.x
    }

    /// Returns the y-coordinate
    pub fn get_y(&self) -> f32 {
        self.y
    }

    /// Creates a new position with explicit x-coordinate
    pub fn with_x(&mut self, x: f32) -> Self {
        Self { x: x, y: self.y }
    }
    /// Creates a new position with explicit y-coordinate
    pub fn with_y(&self, y: f32) -> Self {
        Self { x: self.x, y: y }
    }

    /// Creates a new position with updated x-coordinate
    pub fn move_x(&mut self, amount: f32) -> Self {
        let new_x = (self.x + amount).max(0.0);
        Self {
            x: new_x,
            y: self.y,
        }
    }
}

#[derive(Debug)]
pub enum PositionError {
    NegativeX,
    NegativeY,
}
