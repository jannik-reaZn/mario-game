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
