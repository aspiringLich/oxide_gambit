use bevy::prelude::*;

// constants
const SQ_SIZE: f32 = 64.0; // size of the chess squares
const IMG_SIZE: f32 = 140.0; // size of the images were loading
const SPRITE_SIZE: f32 = SQ_SIZE / IMG_SIZE; // size of the chesspiece sprite

// Z-AXIS:
//
// 1.0  chess pieces
// 0.0  the board

/// returns a vector from a chessboard rank / file
fn vec_from_coord(rank: i8, file: i8) -> Vec3 {
    Vec3::new(
        SQ_SIZE * (rank - 4) as f32,
        SQ_SIZE * (file - 4) as f32,
        0.0,
    )
}

// piece characters from index
const PIECE_CHAR: [char; 6] = ['p', 'r', 'n', 'b', 'k', 'q'];

// draw the chess pieces based on the state
fn draw_chess_pieces(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    state: Res<crate::State>,
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
                        if square.0 >> 7 == 1 { 'b' } else { 'w' },
                        PIECE_CHAR[(square.0 & 0x7F) as usize - 1],
                    )),
                    ..Default::default()
                });
            }
            _ => {
                panic!("Invalid chess board! Or something idek /shrug")
            } // if it gets here something is wrong with your chess board
        }
        i += 1;
    }
}

/// setup the chessboard!
pub fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    state: Res<crate::State>
) {
    // spawn a camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // draw the chessboard squares
    draw_chessboard(&mut commands);

    // draw the pieces
    draw_chess_pieces(&mut commands, asset_server, state);
}

/// draw the squares of the chessboard
fn draw_chessboard(commands: &mut Commands) {
    use crate::theme::*;

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
