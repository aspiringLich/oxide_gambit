use crate::{
    chess_logic::{ChessState, Piece, PieceType, Position},
    interactive::init_interactive,
    render::theme::*,
};
use bevy::prelude::*;

use super::ChessboardSquare;

// constants
pub const SQ_SIZE: f32 = 64.0; // size of the chess squares
pub const IMG_SIZE: f32 = 140.0; // size of the images were loading
pub const SPRITE_SIZE: f32 = SQ_SIZE / IMG_SIZE; // size of the chesspiece sprite

// Z-AXIS:
//
// 2.0  player "held" pieces
// 1.5  obfuscate square
// 1.0  chess pieces
// 0.0  the board

/// returns a vector from a chessboard rank / file
pub fn vec_from_coord(rank: i8, file: i8) -> Vec3 {
    Vec3::new(SQ_SIZE * -3.5 + SQ_SIZE * rank as f32, SQ_SIZE * -3.5 + SQ_SIZE * file as f32, 0.0)
}

/// returns a vector from posz
pub fn vec_from_posz(position: Position, z: f32) -> Vec3 {
    Vec3::new(
        SQ_SIZE * -3.5 + SQ_SIZE * position.x() as f32,
        SQ_SIZE * -3.5 + SQ_SIZE * position.y() as f32,
        z,
    )
}

/// setup the chessboard!
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>, state: Res<ChessState>) {
    // spawn a camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // draw the chessboard squares
    draw_chessboard(&mut commands);

    // render those pieces
    state.render_pieces(&mut commands, &asset_server)
}

/// draw the squares of the chessboard
fn draw_chessboard(commands: &mut Commands) {
    commands
        .spawn_bundle(SpriteBundle { ..default() })
        .insert(ChessboardSquare)
        .insert(Name::new("Chessboard Squares Parent"))
        .with_children(|parent| {
            for pos in 0..64 {
                // check if color should be light or dark
                let color: bool = (pos + (pos / 8)) % 2 == 0;

                // spawn the chessboard Square
                parent
                    .spawn_bundle(SpriteBundle {
                        transform: Transform {
                            translation: vec_from_posz(Position(pos), 0.0),
                            scale: Vec3::new(SQ_SIZE, SQ_SIZE, 0.0),
                            ..default()
                        },
                        sprite: Sprite { color: [SQ_LIGHT, SQ_DARK][color as usize], ..default() },
                        ..default()
                    })
                    .insert(Name::new(format!("Square {}", pos)));
            }
        });
}
