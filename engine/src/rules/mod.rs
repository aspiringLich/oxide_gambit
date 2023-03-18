pub mod fen;
pub mod piece;
pub mod piece_info;

#[derive(Debug)]
pub struct Rules {}

impl Rules {
    pub fn standard() -> Self {
        Self {}
    }
}
