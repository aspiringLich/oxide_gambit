use std::f32::NEG_INFINITY;

use crate::chess_logic::chess_state::ChessState;

impl ChessState {
    pub fn evaluate(&self) -> f32 {
        // piece values
        const value: [f32; 7] = [0.0, 1.0, 5.0, 3.0, 3.0, 0.0, 9.0];

        let mut score: [f32; 2] = [0.0; 2];

        if self.moves.len() == 0 {
            if self.checked() {
                return f32::NEG_INFINITY;
            } else {
                return 0.0;
            }
        }

        for i in 0..=1 {
            for piece in self.pieces[i].iter() {
                score[i] += value[piece.variant() as usize];
            }
        }
        return if self.turn { score[1] - score[0] } else { score[0] - score[1] };
    }
}
