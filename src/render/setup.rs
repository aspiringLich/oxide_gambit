use crate::{
    chess_logic::{ChessState, Piece, PieceType, Pos},
    interactive::init_interactive,
    render::theme::*,
};
use bevy::{prelude::*, utils::Instant};

// constants
pub const SQ_SIZE: f32 = 64.0; // size of the chess squares
const IMG_SIZE: f32 = 140.0; // size of the images were loading
const SPRITE_SIZE: f32 = SQ_SIZE / IMG_SIZE; // size of the chesspiece sprite

// Z-AXIS:
//
// 2.0  player "held" pieces
// 1.5  obfuscate square
// 1.0  chess pieces
// 0.0  the board

/// returns a vector from a chessboard rank / file
fn vec_from_coord(rank: i8, file: i8) -> Vec3 {
    Vec3::new(SQ_SIZE * -3.5 + SQ_SIZE * rank as f32, SQ_SIZE * -3.5 + SQ_SIZE * file as f32, 0.0)
}

/// returns a vector from posz
pub fn vec_from_posz(pos: Pos, z: f32) -> Vec3 {
    Vec3::new(
        SQ_SIZE * -3.5 + SQ_SIZE * pos.x() as f32,
        SQ_SIZE * -3.5 + SQ_SIZE * pos.y() as f32,
        z,
    )
}

// piece characters from index
const PIECE_CHAR: [char; 6] = ['p', 'r', 'n', 'b', 'k', 'q'];

// draw the chess pieces based on the state
fn draw_chess_pieces(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    state: &Res<ChessState>,
) {
    // .enumerate() doesnt work for some reason? "Trait bounds not satisfied"
    let mut i = 0;
    for square in state.board {
        // match the piece type
        match square.0 & 0x7F {
            0 => {}
            1..=6 => {
                commands.spawn_bundle(SpriteBundle {
                    transform: Transform {
                        translation: vec_from_coord(i % 8, i / 8),
                        scale: Vec3::new(SPRITE_SIZE, SPRITE_SIZE, 0.0),
                        ..Default::default()
                    },
                    texture: asset_server.load(&format!(
                        "{}{}.png",
                        if square.team() { 'w' } else { 'b' },
                        PIECE_CHAR[(square.0 & 0x7F) as usize - 1],
                    )),
                    ..Default::default()
                });
            }
            _ => {
                panic!("Invalid chess board! Or something idek")
            } // if it gets here something is wrong with your chess board
        }
        i += 1;
    }
}

/// setup the chessboard!
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>, state: Res<ChessState>) {
    // spawn a camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // draw the chessboard squares
    draw_chessboard(&mut commands);

    // draw the pieces
    draw_chess_pieces(&mut commands, asset_server, &state);
}

/// draw the squares of the chessboard
fn draw_chessboard(commands: &mut Commands) {
    for file in 0..8 {
        for rank in 0..8 {
            // check if color should be light or dark
            let mut color: bool = false;
            if file % 2 == 1 {
                color = !color
            }
            if rank % 2 == 1 {
                color = !color
            }

            // spawn the square
            commands.spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: vec_from_coord(rank, file),
                    scale: Vec3::new(SQ_SIZE, SQ_SIZE, 0.0),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: if color { SQ_LIGHT } else { SQ_DARK },
                    ..Default::default()
                },
                ..Default::default()
            });
        }
    }
}
