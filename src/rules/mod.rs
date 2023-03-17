pub mod piece_info;
pub mod setup;
pub mod piece;


use self::{piece_info::PieceInfo, piece::Piece};
pub struct Rules {
    pub pieces: Vec<Piece>,
}
