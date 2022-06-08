use bevy::prelude::*;

use crate::chess_logic::{ChessState, PieceType, PieceVariant, Position};

use super::{vec_from_coord, DrawnPiece};

// piece characters from index
pub const PIECE_CHAR: [char; 6] = ['p', 'r', 'n', 'b', 'k', 'q'];

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

    pub fn piece_name(&self, n: usize) -> String {
        format!(
            "{} {} ({})",
            ["Black", "White"][self.team() as usize],
            ["Invalid Piece :(((", "Pawn", "Rook", "Knight", "King", "Queen", "Bishop"]
                [self.variant() as usize],
            n
        )
    }
}

impl ChessState {
    pub fn render_pieces(&self, commands: &mut Commands, asset_server: &AssetServer) {
        commands
            .spawn_bundle(SpriteBundle { ..default() })
            .insert(DrawnPiece)
            .insert(Name::new("Pieces Parent"))
            .with_children(|parent| {
                for (i, piece) in self.board.iter().enumerate() {
                    use super::SPRITE_SIZE;

                    if let Some(path) = piece.into_image_path() {
                        parent
                            .spawn_bundle(SpriteBundle {
                                transform: Transform {
                                    translation: vec_from_coord(
                                        (i % 8).try_into().unwrap(),
                                        (i / 8).try_into().unwrap(),
                                    ),
                                    scale: Vec3::new(SPRITE_SIZE, SPRITE_SIZE, 0.0),
                                    ..default()
                                },
                                texture: asset_server.load(&path),
                                ..default()
                            })
                            .insert(Name::new(piece.piece_name(i)));
                    }
                }
            });
    }
}
