use std::collections::VecDeque;

use rustc_hash::FxHashSet;

use super::square::Square;

/// A move from one square to another
pub struct Move {
    pub from: Square,
    pub to: Square,
}

/// Stores the list of moves that can be made
pub struct Moves {
    sliding: FxHashSet<Move>,
    
}