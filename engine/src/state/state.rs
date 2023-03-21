use crate::{
    chess::{index::Index, square::Square, Team},
    move_gen::moves::Moves,
    rules::{piece_info::PieceInfo, Rules},
};
use std::sync::Arc;

use super::board_state::BoardState;

impl Index<PieceInfo> {
    pub fn is_empty(self) -> bool {
        self == Index::new(0)
    }
}

/// A struct representing the state of a chess game
#[derive(Clone, Debug)]
pub struct State {
    /// The rules of the game
    pub rules: Arc<Rules>,
    /// The team whose turn it is
    pub turn: Team,
    /// The state of the board
    pub board_state: BoardState,
    /// The list of moves that can be made
    pub moves: Moves,
}

impl State {
    pub fn new(rules: Rules) -> Self {
        Self {
            rules: Arc::new(rules),
            turn: Team::White,
            board_state: BoardState::new(),
            moves: Moves::new(),
        }
    }

    /// Makes a move on the board
    pub fn make_move(&mut self, from: Square, to: Square) {
        self.board_state.move_piece(from, to);
        self.turn = self.turn.switch();
        self.moves = Moves::generate(&self.board_state);
        // dbg!(self);
    }
}

pub trait StateGet<T> {
    fn get(&self, idx: Index<T>) -> &T;
}
