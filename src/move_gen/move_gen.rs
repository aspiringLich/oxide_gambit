use std::collections::VecDeque;

use rustc_hash::FxHashSet;

use crate::rules::piece::PieceInfo;

use crate::chess::{square::Square, state::Index};

/// A move from one square to another
pub struct Move {
    pub piece: Index<PieceInfo>,
    pub to: Square,
}

/// Stores the list of moves that can be made
pub struct Moves {
    pub sliding: FxHashSet<Move>,
    pub normal: FxHashSet<Move>,
}