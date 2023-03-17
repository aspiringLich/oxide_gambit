pub mod fen;
pub mod piece;
pub mod piece_info;

use self::{piece::Piece, piece_info::PieceInfo};
#[derive(Debug)]
pub struct Rules {}

impl Rules {
    pub fn standard() -> Self {
        Self {}
    }
}
