use bevy::prelude::*;

use crate::chess_logic::{ChessState, PieceType, PieceVariant, Position};

use super::vec_from_coord;

// piece characters from index
pub const PIECE_CHAR: [char; 6] = ['p', 'r', 'n', 'b', 'k', 'q'];

#[derive(Component)]
pub struct DrawnPiece;

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
            commands
                .spawn_bundle(SpriteBundle {
                    transform: Transform {
                        translation: vec_from_coord(
                            position.x().try_into().unwrap(),
                            position.y().try_into().unwrap(),
                        ),
                        scale: Vec3::new(SPRITE_SIZE, SPRITE_SIZE, 0.0),
                        ..default()
                    },
                    texture: asset_server.load(&path),
                    ..default()
                })
                .insert(DrawnPiece);
        }
    }
}

impl ChessState {
    pub fn render_pieces(&self, mut commands: Commands, asset_server: &AssetServer) {
        for (i, piece) in self.board.iter().enumerate() {
            piece.draw(Position::new(i as u8), &mut commands, &asset_server);
        }
    }
}
