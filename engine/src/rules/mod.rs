pub mod piece_info;
pub mod fen;
pub mod piece;


use self::{piece_info::PieceInfo, piece::Piece};
#[derive(Debug)]
pub struct Rules {
}

impl Rules {
    pub fn standard() -> Self {
        Self { }
    }
}
