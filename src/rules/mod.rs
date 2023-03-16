pub mod piece;
pub mod defs;

use self::piece::PieceInfo;
pub struct Rules {
    pub piece_info: Vec<PieceInfo>,
}