pub struct Bounds {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

impl Bounds {
    pub fn new(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }

    pub fn left(&self) -> f32 {
        self.left
    }

    pub fn right(&self) -> f32 {
        self.right
    }

    pub fn top(&self) -> f32 {
        self.top
    }

    pub fn bottom(&self) -> f32 {
        self.bottom
    }

    /// AABB (axis-aligned bounding box) overlap test.
    ///
    /// Two boxes overlap only if their projections overlap on *both* axes.
    /// If there is a gap on either axis, they cannot be touching.
    /// The comparisons are strict, so boxes that merely share an edge do not count.
    pub fn intersects(&self, other: &Bounds) -> bool {
        self.left < other.right
            && self.right > other.left
            && self.top < other.bottom
            && self.bottom > other.top
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unit_box(left: f32, top: f32) -> Bounds {
        Bounds::new(left, left + 10.0, top, top + 10.0)
    }

    #[test]
    fn overlapping_boxes_intersect() {
        assert!(unit_box(0.0, 0.0).intersects(&unit_box(5.0, 5.0)));
    }

    #[test]
    fn contained_box_intersects() {
        let outer = Bounds::new(0.0, 100.0, 0.0, 100.0);
        assert!(outer.intersects(&unit_box(40.0, 40.0)));
    }

    #[test]
    fn gap_on_x_axis_does_not_intersect() {
        assert!(!unit_box(0.0, 0.0).intersects(&unit_box(20.0, 0.0)));
    }

    #[test]
    fn gap_on_y_axis_does_not_intersect() {
        assert!(!unit_box(0.0, 0.0).intersects(&unit_box(0.0, 20.0)));
    }

    #[test]
    fn shared_edge_does_not_intersect() {
        assert!(!unit_box(0.0, 0.0).intersects(&unit_box(10.0, 0.0)));
        assert!(!unit_box(0.0, 0.0).intersects(&unit_box(0.0, 10.0)));
    }

    #[test]
    fn intersection_is_symmetric() {
        let a = unit_box(0.0, 0.0);
        let b = unit_box(5.0, 5.0);
        assert_eq!(a.intersects(&b), b.intersects(&a));
    }
}
