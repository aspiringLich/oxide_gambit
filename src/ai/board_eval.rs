use std::f32::NEG_INFINITY;

use super::*;
use crate::chess_logic::*;

impl Piece {
    pub fn value(&self) -> f32 {
        let out = PIECE_VALUE[self.variant() as usize];
        return if self.team() { out } else { -out };
    }
}

impl ChessState {
    pub fn init_evaluation(&mut self) {
        use PieceVariant::*;

        // check for checkmate and stalemate
        if self.moves.len() == 0 {
            if self.checked() {
                self.static_eval = f32::NEG_INFINITY;
                return;
            } else {
                self.static_eval = 0.0;
                self.inc_eval = 0.0;
                return;
            }
        }

        for i in 0..=1 {
            for piece in self.pieces[i].iter() {
                self.inc_eval += piece.value();
                self.inc_eval += piece.get_square_value(self.endgame);
            }
        }
    }

    pub fn update_evaluation(&mut self, piece: Piece, new_pos: Position, capture: Option<Piece>) {
        use PieceVariant::*;
        let mut score: [f32; 2] = [0.0; 2];

        // check for checkmate and stalemate
        if self.moves.len() == 0 {
            if self.checked() {
                self.static_eval = f32::NEG_INFINITY;
                return;
            } else {
                self.static_eval = 0.0;
                self.inc_eval = 0.0;
                return;
            }
        }

        // get rid of the piece value and square value for a removed piece
        if let Some(piece) = capture {
            self.inc_eval -= piece.value();
            self.inc_eval -= piece.get_square_value(self.endgame);
        }

        // add in the shift over the square value as that piece moved
        self.inc_eval -= piece.get_square_value(self.endgame);
        let new_piece = Piece::new(piece.variant, new_pos);
        self.inc_eval += new_piece.get_square_value(self.endgame);
    }
}
