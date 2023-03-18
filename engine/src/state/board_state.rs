use crate::{
    chess::{
        board::{Board, BoardIndex},
        index::Index,
        square::Square,
    },
    rules::{
        piece::{Piece, PieceInfoTable},
        piece_info::PieceInfo,
    },
};

#[derive(Clone, Debug)]
pub struct BoardState {
    board: Board<Index<Piece>>,
    pieces: [Piece; 64],
}

static mut PIECE_INFO: PieceInfoTable = PieceInfoTable(vec![]);

/// initializes the piece info table
pub fn init() {
    unsafe {
        PIECE_INFO = PieceInfoTable::init();
    }
}

impl BoardState {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            pieces: [Piece::Empty; 64],
        }
    }

    pub fn board(&self) -> &Board<Index<Piece>> {
        &self.board
    }

    pub fn pieces(&self) -> &[Piece] {
        &self.pieces
    }

    pub fn get_idx(&self, idx: Index<Piece>) -> Option<&PieceInfo> {
        let &piece = idx.get(self.pieces());
        self.get_piece(piece)
    }

    pub fn get_piece(&self, piece: Piece) -> Option<&PieceInfo> {
        unsafe { PIECE_INFO[piece as usize].as_ref() }
    }

    pub fn piece_at(&self, pos: Square) -> Option<&PieceInfo> {
        let idx = self.board[pos];
        self.get_idx(idx)
    }

    /// adds a piece to the board
    pub fn add_piece<I: BoardIndex>(&mut self, piece: Piece, pos: I) {
        // assert that the position is empty
        debug_assert_eq!(self.board[pos], Index::new(0));

        // find the first empty slot in the pieces array
        let i = self
            .pieces
            .iter()
            .enumerate()
            .skip(1)
            .find(|(_, p)| **p == Piece::Empty)
            .map(|(i, _)| i)
            .unwrap();
        self.pieces[i] = piece;
        self.board[pos] = Index::new(i as u8);
    }

    /// removes a piece from the board
    pub fn remove_piece<I: BoardIndex>(&mut self, pos: I) {
        // assert that the position is not empty
        debug_assert_ne!(self.board[pos], Index::new(0));

        // find the piece and remove it
        let piece = self.board[pos];
        *piece.get_mut(&mut self.pieces) = Piece::Empty;
        self.board[pos] = Index::new(0);
    }

    pub fn move_piece<I: BoardIndex>(&mut self, from: I, to: I) {
        // assert that the position is not empty
        debug_assert_ne!(self.board[from], Index::new(0));

        // delete the piece at the destination if it exists
        if self.board[to] != Index::new(0) {
            self.remove_piece(to);
        }

        // move the piece
        self.board[to] = self.board[from];
        self.board[from] = Index::new(0);
    }
}
