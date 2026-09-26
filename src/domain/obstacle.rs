use crate::domain::{bounds::Bounds, position::Position, size::Size};

pub struct Obstacle {
    position: Position,
    size: Size,
}

impl Obstacle {
    pub fn new(position: Position, size: Size) -> Self {
        Self { position, size }
    }

    pub fn bounds(&self) -> Bounds {
        Bounds::new(
            self.position.x(),
            self.position.x() + self.size.width(),
            self.position.y(),
            self.position.y() + self.size.height(),
        )
    }
}
