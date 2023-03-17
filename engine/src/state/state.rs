use crate::{
    chess::{
        board::{Board, BoardIndex},
        index::Index,
        Team,
    },
    rules::{
        piece::{Piece, PieceInfoTable},
        piece_info::PieceInfo,
        Rules,
    },
};
use std::{
    cell::{Cell, RefCell},
    hash::Hash,
};

impl Index<PieceInfo> {
    pub fn is_empty(self) -> bool {
        self == Index::new(0)
    }
}

/// A struct representing the state of a chess game
#[derive(Clone, Debug)]
pub struct State<'a> {
    /// Stores all the pieces
    pub piece_info: &'a PieceInfoTable,
    /// Stores all the pieces
    pub pieces: Vec<Piece>,
    /// The rules of the game
    pub rules: &'a RefCell<Rules>,
    /// The team whose turn it is
    pub turn: Team,
    /// The board: the pieces are indexes into `rules.piece_info`
    pub board: Board<Index<Piece>>,
}

static mut PIECE_INFO: PieceInfoTable = PieceInfoTable(vec![]);

impl<'a> State<'a> {
    pub fn new(rules: &'a RefCell<Rules>) -> Self {
        unsafe {
            PIECE_INFO = PieceInfoTable::init();
            Self {
                piece_info: &PIECE_INFO,
                rules,
                pieces: vec![],
                turn: Team::White,
                board: Board::new(),
            }
        }
    }

    pub fn piece_at<T: BoardIndex>(&self, square: T) -> &Option<PieceInfo> {
        &self.piece_info[*self.board[square].get(&*self.pieces) as usize]
    }

    pub fn get_piece(&self, idx: Index<Piece>) -> &Option<PieceInfo> {
        &self.piece_info[*idx.get(&self.pieces) as usize]
    }
}

pub trait StateGet<T> {
    fn get(&self, idx: Index<T>) -> &T;
}
