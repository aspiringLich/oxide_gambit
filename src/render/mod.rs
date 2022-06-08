pub mod display_piece;
pub mod setup;
pub mod theme;

pub use display_piece::*;
pub use setup::*;
pub use theme::*;

use bevy::prelude::Component;

#[derive(Component)]
struct ChessboardSquare;

#[derive(Component)]
pub struct DrawnPiece;
