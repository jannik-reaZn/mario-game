pub struct Position(f32, f32);

impl Position {
    pub fn new(x: f32, y: f32) -> Result<Self, String> {
        if x < 0.0 {
            return Err("The x position cannot be negative".to_string());
        } else if y < 0.0 {
            return Err("The y position cannot be negative".to_string());
        }
        Ok(Self(x, y))
    }

    pub fn get_x(&self) -> f32 {
        self.0
    }

    pub fn get_y(&self) -> f32 {
        self.1
    }

    pub fn move_by(&self, dx: f32, dy: f32) -> Result<Self, String> {
        Self::new(self.0 + dx, self.1 + dy)
    }
}
