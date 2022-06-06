use std::cmp::max;

use bevy::prelude::default;

use super::{
    chess_state::ChessState,
    position::{coord_to_index, is_45},
    Piece,
};

pub enum PinType {
    Pinned,           // a piece is completely pinned
    PinDir((u8, u8)), // a piece can still move along this direction (and its inverse)
    None,             // a piece is not pinned
}

impl Default for PinType {
    fn default() -> Self {
        PinType::None
    }
}

impl ChessState {
    pub fn king(&self, turn: bool) {}

    pub fn check_pins(&mut self) {
        let pieces: &[Vec<Piece>; 2] = unsafe { std::mem::transmute(&self.pieces) };

        // closest pieces
        let closest: [[(Piece, i8); 9]; 2] = [[(default(), i8::MAX); 9]; 2];

        for piece in &pieces[0] {
            let (x, y) = piece.rel_from(self.king_position[self.turn as usize]);
            if is_45(x, y) {
                let max = max(x, y);
                let index = coord_to_index(x, y);
            }
        }
    }
}
