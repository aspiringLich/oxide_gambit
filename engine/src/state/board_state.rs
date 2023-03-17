use crate::{
    chess::{board::Board, index::Index, square::Square},
    rules::piece::Piece,
};

pub struct BoardState<const SIZE: usize> {
    board: Board<Index<Piece>>,
    pieces: [Piece; SIZE],
}

impl<const SIZE: usize> BoardState<SIZE> {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            pieces: [Piece::Empty; SIZE],
        }
    }

    pub fn board(&self) -> &Board<Index<Piece>> {
        &self.board
    }

    /// adds a piece to the board
    pub fn add_piece(&mut self, piece: Piece, pos: Square) {
        // assert that the position is empty
        debug_assert_eq!(self.board[pos], Index::new(0));

        // find the first empty slot in the pieces array
        let empty = self
            .pieces
            .iter_mut()
            .skip(1)
            .find(|x| **x == Piece::Empty)
            .unwrap();
        *empty = piece;
    }
}
