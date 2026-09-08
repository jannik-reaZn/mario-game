use macroquad::color::Color;

pub const GRAVITY: f32 = 9.81 * 100.0;
pub const JUMP_STRENGTH: f32 = 500.0;
// For now body is circular
pub struct Player {
    pub radius: f32,
    pub velocity: f32,
    pub position: (f32, f32),
    pub color: Color, // TODO this needs to be String instead
}

/// Calculates the next y position after each frame.
/// The players's velocity starts negative (upward) and continously increases.
/// Eventually vy reaches zero at the top of the jump.
/// The velocity becomes positive (downward).
///
/// y: The position on y-axis
/// dt: Delta time
///
/// Example:
/// ↑
//  |       *
//  |      * *
//  |     *   *
//  |    *     *
//  |   *       *
//  |  *         *
//  | *           *
//  |*             *
//──┴──────────────── ground
impl Player {
    pub fn calculate_y_position(&mut self, dt: f32) {
        self.velocity += GRAVITY * dt;
        self.position.1 += self.velocity * dt;
    }
}
