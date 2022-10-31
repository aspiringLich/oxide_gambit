use super::{moves::Moves, pieces::Piece, square::Square};
use core::fmt::Debug;

/// Stores the state of the chess board at any one time
pub struct State {
    /// the pieces
    board: [Option<Piece>; 64],
    moves: Vec<Box<Moves>>,
    start_index: [u8; 6],
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("State")
            .field("board", &self.board)
            .field("moves", &self.moves)
            .field("start_index", &self.start_index)
            .finish()
    }
}
