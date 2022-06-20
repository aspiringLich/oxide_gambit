use std::f32::NEG_INFINITY;

use crate::chess_logic::*;

impl Piece {
    #[inline]
    pub fn value(&self) -> f32 {
        use PieceVariant::*;

        match self.variant() {
            None => unreachable!(),
            Pawn => 1.0,
            Rook => 5.0,
            Knight => 3.2,
            Bishop => 3.3,
            King => 0.0,
            Queen => 9.0,
        }
    }
}

impl ChessState {
    pub fn evaluate(&self) -> f32 {
        let mut score: [f32; 2] = [0.0; 2];

        // check for checkmate and stalemate
        if self.moves.len() == 0 {
            if self.checked() {
                return f32::NEG_INFINITY;
            } else {
                return 0.0;
            }
        }

        for i in 0..=1 {
            for piece in self.pieces[i].iter() {
                score[i] += piece.value();
                score[i] += piece.get_square_value(self.endgame);
            }
        }
        return if self.turn { score[1] - score[0] } else { score[0] - score[1] };
    }
}
