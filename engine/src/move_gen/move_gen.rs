use std::default::default;

use rustc_hash::FxHashSet;

use crate::chess::index::Index;
use crate::rules::piece::Piece;

use crate::chess::square::Square;

/// A move from one square to another
#[derive(Hash, PartialEq, Eq)]
pub struct Move {
    pub piece: Index<Piece>,
    pub to: Square,
}

/// Stores the list of moves that can be made
pub struct Moves {
    pub moves: FxHashSet<Move>,
}

impl Moves {
    pub fn new() -> Self {
        Self { moves: default() }
    }

    /// Inserts a move into the list of moves
    pub fn insert(&mut self, piece: Index<Piece>, to: Square) {
        self.moves.insert(Move { piece, to });
    }

    /// Inserts a *good* move into the list of moves
    pub fn insert_good(&mut self, piece: Index<Piece>, to: Square) {
        self.moves.insert(Move { piece, to });
    }
}
