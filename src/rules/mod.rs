pub mod piece_info;
pub mod setup;
pub mod piece;


use self::piece_info::PieceInfo;
pub struct Rules {
    pub piece_info: Vec<PieceInfo>,
}

impl Rules {
    pub fn add_piece(mut self, piece: PieceInfo) -> Self {
        self.piece_info.push(piece);
        self
    }
}
