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

    // Getters
    pub fn get_x(&self) -> f32 {
        self.0
    }

    pub fn get_y(&self) -> f32 {
        self.1
    }

    // Setters
    pub fn set_x(&mut self, x: f32) {
        self.0 = x;
    }

    pub fn set_y(&mut self, y: f32) {
        self.1 = y;
    }

    pub fn move_x(&mut self, amount: f32) {
        self.0 = (self.0 + amount).max(0.0);
    }
}

#[derive(Debug)]
pub enum PositionError {
    NegativeX,
    NegativeY,
}
