pub struct Size {
    width: f32,
    height: f32,
}

impl Size {
    pub fn new(width: f32, height: f32) -> Result<Self, SizeError> {
        if width < 0.0 {
            return Err(SizeError::NegativeWidth);
        } else if height < 0.0 {
            return Err(SizeError::NegativeHeight);
        }
        Ok(Self { width, height })
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }
}

#[derive(Debug)]
pub enum SizeError {
    NegativeWidth,
    NegativeHeight,
}
