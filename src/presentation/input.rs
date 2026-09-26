use crate::domain::movement::PlayerMovement;
use macroquad::input::{KeyCode, is_key_down, is_key_pressed};

/// Translates the keyboard state of this frame into the movements the player
/// wants to perform.
///
/// Left and right are *held* keys (they fire every frame while down), jump is
/// a *press* (it fires once per key stroke, so holding Space does not bounce).
pub fn read_player_movements() -> Vec<PlayerMovement> {
    let mut movements = Vec::new();

    if is_key_down(KeyCode::Right) {
        movements.push(PlayerMovement::Right);
    }
    if is_key_down(KeyCode::Left) {
        movements.push(PlayerMovement::Left);
    }
    if is_key_pressed(KeyCode::Space) {
        movements.push(PlayerMovement::Jump);
    }

    movements
}
