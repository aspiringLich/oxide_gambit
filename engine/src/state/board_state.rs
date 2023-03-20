use strum::IntoEnumIterator;

use crate::{
    chess::{
        board::{Board, BoardIndex},
        index::Index,
        square::Square,
    },
    rules::{
        def_standard::Invalid,
        piece::{Piece, PieceTrait},
        piece_info::PieceInfo,
    },
};

#[derive(Clone, Debug)]
pub struct BoardState {
    board: Board<Index<Piece>>,
    pieces: [Piece; 64],
}

pub static mut PIECE_INFO: Vec<Option<PieceInfo>> = vec![];
static mut PIECES: Vec<Box<dyn PieceTrait>> = vec![];

/// initializes the piece info table
pub fn init() {
    unsafe {
        PIECE_INFO = Piece::iter().map(|p| p.info()).collect();
        PIECES = Piece::iter()
            .map(|p| p.piece().unwrap_or(Box::new(Invalid)))
            .collect();
    }
}

pub trait GetPiece {
    fn get_piece(&self, state: &BoardState) -> Piece;
}

impl GetPiece for Piece {
    fn get_piece(&self, _: &BoardState) -> Piece {
        *self
    }
}

impl GetPiece for Index<Piece> {
    fn get_piece(&self, state: &BoardState) -> Piece {
        *self.get(state.pieces())
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

    pub fn get_info<T: GetPiece>(&self, piece: T) -> Option<&PieceInfo> {
        unsafe { PIECE_INFO[piece.get_piece(self) as usize].as_ref() }
    }

    pub fn get_piece<T: GetPiece>(&self, piece: T) -> &'static dyn PieceTrait {
        unsafe { PIECES[piece.get_piece(self) as usize].as_ref() }
    }

    pub fn info_at(&self, pos: Square) -> Option<&PieceInfo> {
        let idx = self.board[pos];
        self.get_info(idx)
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
