pub struct Radius {
    value: f32,
}

impl Radius {
    pub fn new(radius: f32) -> Result<Self, RadiusError> {
        if radius < 0.0 {
            return Err(RadiusError::RadiusNegativ);
        }
        Ok(Self { value: radius })
    }

    pub fn value(&self) -> f32 {
        self.value
    }
}

#[derive(Debug)]
pub enum RadiusError {
    RadiusNegativ,
}
