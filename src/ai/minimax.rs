use bevy::prelude::default;

use crate::chess_logic::*;

struct PruningInfo {
    least_worst: [f32; 2], // the "alpha" and "beta", the least worst move of any given node found so far
}

impl Default for PruningInfo {
    fn default() -> Self {
        Self { least_worst: [f32::INFINITY; 2] }
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
        let mut max_index = 0;
        let mut max_val = f32::NEG_INFINITY;

        assert!(depth >= 1);

        let mut info: PruningInfo = default();

        for (i, item) in self.moves.iter().enumerate() {
            // negate as this will return the best move from the other team's point of view
            let value = -self.make_move(*item).minimax(depth - 1, &mut info);
            if value > max_val {
                max_val = value;
                max_index = i;
            }
            // dbg!(max_val);
        }
        return self.moves[max_index];
    }

    /// run the minimax algorithm on a chess state to a specified depth
    fn minimax(&self, depth: usize, info: &mut PruningInfo) -> f32 {
        // dbg!(self);
        // if depth is zero, return the move
        if depth == 0 {
            return self.evaluation();
        }

        let mut max_val = f32::NEG_INFINITY;
        let mut min_val = f32::INFINITY;

        let least_worst = info.least_worst[self.turn()];

        for &item in &self.moves {
            // negate as this will return the best move from the other team's point of view
            let mut value = self.make_move(item).minimax(depth - 1, info);
            if !self.turn {
                value = -value
            }

            // alpha beta pruning
            if value < least_worst {
                return value;
            }

            if value > max_val {
                max_val = value;
            } else if value < min_val {
                min_val = value;
            }
        }

        if min_val < info.least_worst[self.turn()] {
            info.least_worst[self.turn()] = min_val;
        }
        return max_val;
    }
}
