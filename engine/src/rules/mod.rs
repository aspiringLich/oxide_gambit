pub mod fen;
pub mod piece;
pub mod piece_info;
pub mod def_standard;

#[derive(Debug)]
pub struct Rules {}

impl Rules {
    pub fn standard() -> Self {
        Self {}
    }
}
