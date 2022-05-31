use bevy::prelude::*;

use crate::chess_logic::Position;

pub struct WindowInfo {
    pub size: Vec2,
    pub cursor_pos: Vec2,
}

impl WindowInfo {
    pub fn empty() -> Self {
        WindowInfo { size: Vec2::new(0.0, 0.0), cursor_pos: Vec2::new(0.0, 0.0) }
    }

    pub fn chessboard_pos(&self) -> Option<Position> {
        use crate::render::setup::SQ_SIZE;
        // if the cursor is within the bounds of the chessboard
        if self.cursor_pos.x > self.size.x / 2.0 - SQ_SIZE * 4.0
            && self.cursor_pos.x < self.size.x / 2.0 + SQ_SIZE * 4.0
            && self.cursor_pos.y < self.size.y / 2.0 + SQ_SIZE * 4.0
            && self.cursor_pos.y > self.size.y / 2.0 - SQ_SIZE * 4.0
        {
            // send where the mouse clicked on the chessboard
            let x = (self.cursor_pos.x - self.size.x / 2.0 + SQ_SIZE * 4.0) / SQ_SIZE;
            let y = (self.cursor_pos.y - self.size.y / 2.0 + SQ_SIZE * 4.0) / SQ_SIZE;
            return Some(Position(x as u8 + y as u8 * 8));
            //dbg!(x as u8 + y as u8 * 8);
        }
        None
    }
}

pub fn update_window_info(mut info: ResMut<WindowInfo>, window: Res<Windows>) {
    info.size.x = window.get_primary().unwrap().width();
    info.size.y = window.get_primary().unwrap().height();
    let err = window.get_primary().unwrap().cursor_position();
    if err.is_none() {
        return;
    }
    info.cursor_pos = err.unwrap();
}
