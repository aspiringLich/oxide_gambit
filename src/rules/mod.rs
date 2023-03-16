pub mod piece;

use crate::{chess::Team, move_gen};

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
