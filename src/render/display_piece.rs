use bevy::prelude::*;

use crate::chess_logic::{PieceType, PieceVariant, Position};

use super::vec_from_coord;

// piece characters from index
const PIECE_CHAR: [char; 6] = ['p', 'r', 'n', 'b', 'k', 'q'];

impl PieceType {
    // return the image path of the given piece
    pub fn into_image_path(&self) -> Option<String> {
        if self.variant() == PieceVariant::None {
            return None;
        }
        Some(format!(
            "{}{}.png",
            ['b', 'w'][self.team() as usize],
            PIECE_CHAR[self.variant() as usize - 1],
        ))
    }

    // draw a chess piece
    pub fn draw(&self, position: Position, commands: &mut Commands, asset_server: &AssetServer) {
        use super::SPRITE_SIZE;

        if let Some(path) = self.into_image_path() {
            commands.spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: vec_from_coord(
                        position.x().try_into().unwrap(),
                        position.y().try_into().unwrap(),
                    ),
                    scale: Vec3::new(SPRITE_SIZE, SPRITE_SIZE, 0.0),
                    ..Default::default()
                },
                texture: asset_server.load(&path),
                ..Default::default()
            });
        }
    }
}
