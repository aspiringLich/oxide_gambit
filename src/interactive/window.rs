use bevy::math::Vec2;
use bevy::prelude::{Res, ResMut};
use bevy::window::{Window, Windows};

pub struct WindowInfo {
    pub size: Vec2,
    pub cursor_pos: Vec2,
}

impl WindowInfo {
    pub fn empty() -> Self {
        WindowInfo {
            size: Vec2::new(0.0, 0.0),
            cursor_pos: Vec2::new(0.0, 0.0),
        }
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
