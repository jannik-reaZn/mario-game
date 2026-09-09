#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Velocity {
    value: f32,
}

impl Velocity {
    pub fn new(value: f32) -> Self {
        Self { value }
    }

    /// Returns the velocity
    pub fn value(&self) -> f32 {
        self.value
    }
    /// Creates a new velocity with updated velocity
    pub fn accelerate(&self, acceleration: f32, dt: f32) -> Self {
        Self {
            value: self.value + acceleration * dt,
        }
    }

    /// Created a new velocity with 0.0
    pub fn stationary() -> Self {
        Self { value: 0.0 }
    }
}
