use std::f32::NEG_INFINITY;

use bevy::prelude::default;

use crate::chess_logic::*;

struct PruningInfo {
    alpha: f32,
    beta: f32,
}

impl Default for PruningInfo {
    fn default() -> Self {
        Self { alpha: NEG_INFINITY, beta: f32::INFINITY }
    }
}

impl PruningInfo {
    /// update the necessary info for the next depth
    pub fn update(&self) -> Self {
        Self { alpha: -self.beta, beta: -self.alpha }
    }
}

impl ChessState {
    /// make a move and return a copy
    pub fn make_move(&self, chess_move: ChessMove) -> ChessState {
        let mut out: ChessState = self.clone();
        out.excecute_move(
            Piece::new(self.at(chess_move.origin), chess_move.origin),
            chess_move.target,
        );
        return out;
    }

    /// the top level algorithm that will get the chess move as well
    pub fn run_minimax(&self, depth: usize) -> ChessMove {
        let mut best_index = 0;
        let mut best_score = f32::NEG_INFINITY;

        assert!(depth >= 1);

        let mut info: PruningInfo = default();

        for (i, item) in self.moves.iter().enumerate() {
            let score = -self.make_move(*item).minimax(depth - 1, info.update());

            if score > best_score {
                best_index = i;
                best_score = score;
                if score > info.alpha {
                    info.alpha = score;
                }
            }
        }
        eprintln!(
            "Chose move with evaluation of {} ({:+})",
            best_score,
            self.evaluation() + best_score
        );
        return self.moves[best_index];
    }

    /// run the minimax algorithm on a chess state to a specified depth
    fn minimax(&self, depth: usize, mut info: PruningInfo) -> f32 {
        // dbg!(self);
        // if depth is zero, return the move
        if depth == 0 {
            let out = self.evaluation();
            return if self.turn { out } else { -out };
        }

        let mut best_score: f32 = NEG_INFINITY;

        for &item in &self.moves {
            let score = -self.make_move(item).minimax(depth - 1, info.update());

            if score >= info.beta {
                return info.beta;
            }
            if score > best_score {
                best_score = score;
                if score > info.alpha {
                    info.alpha = score;
                }
            }
        }

        return info.alpha;
    }
}
