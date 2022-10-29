use std::f32::NEG_INFINITY;

use bevy::prelude::default;

use crate::chess_logic::*;

use super::{transposition::TranspositionTable, *};

struct PruningInfo {
    alpha: f32,
    beta: f32,
}

struct MutableInfo {
    nodes_searched: usize,
    branches_pruned: usize,
}

impl PruningInfo {
    fn new() -> Self {
        // i give up trying to be safe this is just less headache
        Self { alpha: NEG_INFINITY, beta: f32::INFINITY }
    }

    /// update the necessary info for the next depth
    pub fn update(&mut self) -> Self {
        Self { alpha: -self.beta, beta: -self.alpha }
    }
}

impl MutableInfo {
    fn new() -> Self {
        Self { nodes_searched: 0, branches_pruned: 0 }
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

        let mut mutable = MutableInfo::new();
        let mut info = PruningInfo::new();

        for (i, item) in self.moves.iter().enumerate() {
            let score = -self.make_move(*item).minimax(depth - 1, info.update(), &mut mutable);

            if score > best_score {
                best_index = i;
                best_score = score;
                if score > info.alpha {
                    info.alpha = score;
                }
            }
        }
        eprintln!(
            "Chose move with evaluation of {} ({:+})\n\
            {} Nodes Searched\n\
            {} Branches Pruned",
            best_score,
            self.get_static_evaluation() + best_score,
            mutable.nodes_searched,
            mutable.branches_pruned,
        );
        return self.moves[best_index];
    }

    /// run the minimax algorithm on a chess state to a specified depth
    fn minimax(&self, depth: usize, mut info: PruningInfo, mutable: &mut MutableInfo) -> f32 {
        // if this board is already in the table, return
        // dbg!(self);
        // if depth is zero, return the move
        if depth == 0 {
            mutable.nodes_searched += 1;
            let out = self.get_static_evaluation();
            return if self.turn { out } else { -out };
        }

        let mut best_score: f32 = NEG_INFINITY;

        for &item in &self.moves {
            let updated = self.make_move(item);
            let score = -updated.minimax(depth - 1, info.update(), mutable);

            if score >= info.beta {
                mutable.branches_pruned += 1;
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
