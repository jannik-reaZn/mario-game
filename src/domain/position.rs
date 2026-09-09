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
    pub fn x(&self) -> f32 {
        self.x
    }

    /// Returns the y-coordinate
    pub fn y(&self) -> f32 {
        self.y
    }

    /// Creates a new position with explicit x-coordinate
    pub fn with_x(&self, x: f32) -> Result<Position, PositionError> {
        Position::new(x, self.y)
    }
    /// Creates a new position with explicit y-coordinate
    pub fn with_y(&self, y: f32) -> Result<Position, PositionError> {
        Position::new(self.x, y)
    }

    /// Creates a new position with updated x-coordinate
    pub fn move_x(&self, amount: f32) -> Result<Self, PositionError> {
        let new_x = self.x + amount;
        Position::new(new_x, self.y)
    }
}

#[derive(Debug)]
pub enum PositionError {
    NegativeX,
    NegativeY,
}
