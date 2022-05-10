use bevy::{
    input::Input,
    prelude::{EventWriter, MouseButton, Res},
};

use crate::chess_logic::Pos;

use super::WindowInfo;

pub enum MouseEvent {
    PressChessboard(Pos),
}

/// processes mouse events and sends them
pub fn send_mouse_events(
    mouse_click: Res<Input<MouseButton>>,
    mut mouse_ev: EventWriter<MouseEvent>,
    window: Res<WindowInfo>,
) {
    use MouseEvent::*;

    if mouse_click.just_pressed(MouseButton::Left) {
        // check if the click is in the chessboard
        let clicked = cursor_square(&window);
        if clicked.is_none() {
            return;
        }
        mouse_ev.send(PressChessboard(clicked.unwrap()))
    }
}

pub fn cursor_square(window: &Res<WindowInfo>) -> Option<Pos> {
    use crate::render::setup::SQ_SIZE;
    // if the cursor is within the bounds of the chessboard
    if window.cursor_pos.x > window.size.x / 2.0 - SQ_SIZE * 4.0
        && window.cursor_pos.x < window.size.x / 2.0 + SQ_SIZE * 4.0
        && window.cursor_pos.y < window.size.y / 2.0 + SQ_SIZE * 4.0
        && window.cursor_pos.y > window.size.y / 2.0 - SQ_SIZE * 4.0
    {
        // send where the mouse clicked on the chessboard
        let x = (window.cursor_pos.x - window.size.x / 2.0 + SQ_SIZE * 4.0) / SQ_SIZE;
        let y = (window.cursor_pos.y - window.size.y / 2.0 + SQ_SIZE * 4.0) / SQ_SIZE;
        return Some(Pos(x as u8 + y as u8 * 8));
        //dbg!(x as u8 + y as u8 * 8);
    }
    None
}
