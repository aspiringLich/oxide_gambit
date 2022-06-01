use bevy::{
    input::Input,
    prelude::{EventWriter, MouseButton, Res},
};

use crate::chess_logic::Position;

use super::WindowInfo;

#[derive(Debug)]
pub enum MouseEvent {
    PressChessboard(Position),
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

pub fn cursor_square(window: &Res<WindowInfo>) -> Option<Position> {
    use crate::render::setup::SQ_SIZE;
    // if the cursor is within the bounds of the chessboard
    let (x, y) = (window.cursor_pos.x, window.cursor_pos.y);
    let (width, height) = (window.size.x, window.size.y);

    let (xmin, xmax) = ((width / 2. - SQ_SIZE * 4.), (width / 2. + SQ_SIZE * 4.));
    let (ymin, ymax) = ((height / 2. - SQ_SIZE * 4.), (height / 2. + SQ_SIZE * 4.));
    if x >= xmin && x <= xmax && y >= ymin && y <= ymax {
        // send where the mouse clicked on the chessboard
        let x = (x - xmin) / SQ_SIZE;
        let y = (y - ymin) / SQ_SIZE;
        return Some(Position(x as u8 + y as u8 * 8));
        //dbg!(x as u8 + y as u8 * 8);
    }
    None
}
