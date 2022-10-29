use std::hash::*;

use crate::chess_logic::chess_state::ChessState;

pub struct ZobristHasher;

impl Hasher for ZobristHasher {
    fn finish(&self) -> u64 {
        todo!()
    }

    fn write(&mut self, bytes: &[u8]) {
        todo!()
    }
}
