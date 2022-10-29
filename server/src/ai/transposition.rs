use crate::chess_logic::chess_state::ChessState;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

impl Hash for ChessState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for piece in self.board {
            piece.hash(state);
        }
        self.turn.hash(state);
        self.castling.hash(state);
        self.en_passant.hash(state);
    }
}

impl PartialEq for ChessState {
    fn eq(&self, other: &Self) -> bool {
        self.board == other.board
            && self.turn == other.turn
            && self.castling == other.castling
            && self.en_passant == other.en_passant
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Eq for ChessState {}

#[derive(Debug)]
pub struct TranspositionTable {
    table: HashSet<ChessState>,
}

impl TranspositionTable {
    pub fn new() -> Self {
        Self { table: HashSet::new() }
    }

    pub fn contains(&self, board: &ChessState) -> bool {
        self.table.contains(board)
    }

    pub fn insert(&mut self, board: ChessState) {
        self.table.insert(board);
    }
}
