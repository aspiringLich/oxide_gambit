use std::collections::HashSet;

use super::{pieces::Piece, square::Square};

/// Stores the state of the chess board at any one time
///   - both board wise and piece wise representation
pub struct State {
    board: [Option<Piece>; 64],
    moves: [HashSet<Square>; 12],
}
