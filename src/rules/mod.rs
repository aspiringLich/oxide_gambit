pub mod piece;

use crate::{chess::Team, move_gen};
use crate::chess::direction::Direction;

use self::piece::PieceInfo;
pub struct Rules {
    pub piece_info: Vec<PieceInfo>,
}

pub fn white_pawn() -> PieceInfo {
    PieceInfo::new()
        .ch("♟︎")
        .value(1)
        .team(Team::White)
        .move_gen(&move_gen::white_pawn)
}

pub fn black_pawn() -> PieceInfo {
    PieceInfo::new()
        .ch("♟︎")
        .value(1)
        .team(Team::Black)
        .move_gen(&move_gen::black_pawn)
}

pub fn rook() -> PieceInfo {
    PieceInfo::new()
        .ch("♜")
        .value(5)
        .attacks(&Direction::ORTHOGONAL)
}

pub fn knight() -> PieceInfo {
    PieceInfo::new()
        .ch("♞")
        .value(3)
        .move_gen(&move_gen::knight)
}

pub fn bishop() -> PieceInfo {
    PieceInfo::new()
        .ch("♝")
        .value(3)
        .attacks(&Direction::DIAGONAL)
}

pub fn queen() -> PieceInfo {
    PieceInfo::new()
        .ch("♛")
        .value(9)
        .attacks(&Direction::ALL)
}

pub fn king() -> PieceInfo {
    PieceInfo::new()
        .ch("♚")
        .value(0)
        .move_gen(&move_gen::king)
}