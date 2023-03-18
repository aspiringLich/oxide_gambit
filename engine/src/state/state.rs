use crate::{
    chess::{index::Index, Team},
    rules::{piece_info::PieceInfo, Rules}, move_gen::moves::Moves,
};
use std::cell::RefCell;

use super::board_state::BoardState;

impl Index<PieceInfo> {
    pub fn is_empty(self) -> bool {
        self == Index::new(0)
    }
}

/// A struct representing the state of a chess game
#[derive(Clone, Debug)]
pub struct State<'a> {
    /// The rules of the game
    pub rules: &'a RefCell<Rules>,
    /// The team whose turn it is
    pub turn: Team,
    /// The state of the board
    pub board_state: BoardState,
    /// The list of moves that can be made
    pub moves: Moves,
}

impl<'a> State<'a> {
    pub fn new(rules: &'a RefCell<Rules>) -> Self {
        Self {
            rules,
            turn: Team::White,
            board_state: BoardState::new(),
            moves: Moves::new(),
        }
    }
}

pub trait StateGet<T> {
    fn get(&self, idx: Index<T>) -> &T;
}
