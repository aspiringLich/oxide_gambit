use std::collections::VecDeque;
use std::default::default;

use rustc_hash::FxHashSet;

use crate::rules::piece::PieceInfo;

use crate::chess::{square::Square, state::Index};

/// A move from one square to another
#[derive(Hash, PartialEq, Eq)]
pub struct Move {
    pub piece: Index<PieceInfo>,
    pub to: Square,
}

/// Stores the list of moves that can be made
pub struct Moves {
    pub moves: FxHashSet<Move>,
}

impl Moves {
    pub fn new() -> Self {
        Self {
            moves: default(),
        }
    }
    
    /// Inserts a move into the list of moves
    pub fn insert(&mut self, piece: Index<PieceInfo>, to: Square) {
        self.moves.insert(Move { piece, to });
    }
    
    
    /// Inserts a *good* move into the list of moves
    pub fn insert_good(&mut self, piece: Index<PieceInfo>, to: Square) {
        self.moves.insert(Move { piece, to });
    }
}
