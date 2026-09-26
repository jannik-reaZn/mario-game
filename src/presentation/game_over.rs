use crate::presentation::window::{get_screen_height, get_screen_width};
use macroquad::math::vec2;
use macroquad::ui::{hash, root_ui, widgets};

/// Draws the "Game over" pop-up in the middle of the screen.
///
/// Returns `true` in the frame in which the player clicked "Retry". What
/// happens next is not decided here, the caller owns the game.
pub fn draw_game_over() -> bool {
    let size = vec2(260.0, 110.0);
    let position = vec2(
        (get_screen_width() - size.x) / 2.0,
        (get_screen_height() - size.y) / 2.0,
    );

    let mut retry_clicked = false;
    widgets::Window::new(hash!(), position, size)
        .label("Game over")
        .movable(false)
        .close_button(false)
        .ui(&mut root_ui(), |ui| {
            retry_clicked = ui.button(None, "Retry");
        });

    retry_clicked
}
