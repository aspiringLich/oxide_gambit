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
    /// is this piece threatened
    pub fn threatened(&self, piece: Piece) -> bool {
        self.threatened[!piece.team() as usize].squares[piece.position.int()] > 0
    }

    /// is this piece protected
    pub fn protected(&self, piece: Piece) -> bool {
        self.threatened[piece.team() as usize].squares[piece.position.int()] > 0
    }

    /// regenerate the static evaluation and output the eval
    pub fn get_static_evaluation(&self) -> f32 {
        let mut static_eval = 0.0;
        // go through white and if a piece is threatened but not protected, or vice versa do smth
        for piece in &self.pieces[1] {
            let threatened = self.threatened(*piece);
            let protected = self.protected(*piece);
            if threatened ^ protected {
                match threatened {
                    true => static_eval -= piece.value() * THREATENED_WEIGHT,
                    false => static_eval += piece.value() * PROTECTED_WEIGHT,
                }
            }
        }
        // dbg!(static_eval);
        // same but flipped
        for piece in &self.pieces[0] {
            let threatened = self.threatened(*piece);
            let protected = self.protected(*piece);
            if threatened ^ protected {
                match threatened {
                    true => static_eval -= piece.value() * THREATENED_WEIGHT,
                    false => static_eval += piece.value() * PROTECTED_WEIGHT,
                }
            }
        }
        // dbg!(static_eval);
        return self.inc_eval + static_eval;
    }

    pub fn init_evaluation(&mut self) {
        use PieceVariant::*;

        // check for checkmate and stalemate
        if self.moves.len() == 0 {
            if self.checked() {
                self.inc_eval = f32::NEG_INFINITY;
            } else {
                self.inc_eval = 0.0;
            }
            self.done = true;
            return;
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
                self.inc_eval = f32::NEG_INFINITY;
            } else {
                self.inc_eval = 0.0;
            }
            self.done = true;
            return;
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
