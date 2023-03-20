use bevy::prelude::{Color, Commands, Resource};

const fn rgb_hex(color: u32) -> Color {
    let r = (color >> 16) & 0xFF;
    let g = (color >> 8) & 0xFF;
    let b = color & 0xFF;

    Color::Rgba {
        red: r as f32 / 255.0,
        green: g as f32 / 255.0,
        blue: b as f32 / 255.0,
        alpha: 1.0,
    }
}

#[derive(Clone, Resource)]
pub struct Theme {
    name: &'static str,
    pub square: [Color; 2],
    pub piece: [Color; 2],
}

impl Theme {
    pub const fn new(name: &'static str, pc_d: u32, pc_l: u32, sq_d: u32, sq_l: u32) -> Self {
        Self {
            name,
            square: [rgb_hex(sq_d), rgb_hex(sq_l)],
            piece: [rgb_hex(pc_d), rgb_hex(pc_l)],
        }
    }
}

pub const THEMES: &[Theme] = &[
    Theme::new("Default", 0x565352, 0xf9f9f9, 0x769656, 0xeeeed2), //
];

pub fn init(mut commands: Commands) {
    commands.insert_resource(THEMES[0].clone());
}