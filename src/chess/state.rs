use crate::rules::{
    piece::{Piece, PieceInfoTable},
    piece_info::PieceInfo,
    Rules,
};
use std::hash::Hash;

use super::{
    board::{Board, BoardIndex},
    Team,
};

/// A u8 that serves as an index into a list of `T`
#[derive(Debug)]
pub struct Index<T>(u8, std::marker::PhantomData<T>);

impl<T> Hash for Index<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl<T> PartialEq for Index<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for Index<T> {}

impl<T> Clone for Index<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Index<T> {}

impl<T> Default for Index<T> {
    fn default() -> Self {
        Self(0, std::marker::PhantomData)
    }
}

impl<T> Index<T> {
    pub fn get(self, arr: &[T]) -> &T {
        &arr[self.0 as usize]
    }

    pub fn new(idx: u8) -> Self {
        Self(idx, std::marker::PhantomData)
    }
}

impl Index<PieceInfo> {
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }
}

/// A struct representing the state of a chess game
#[derive(Clone)]
pub struct State<'a> {
    /// Stores all the pieces
    pub piece_info: PieceInfoTable,
    /// The rules of the game
    pub rules: &'a Rules,
    /// The team whose turn it is
    pub turn: Team,
    /// The board: the pieces are indexes into `rules.piece_info`
    pub board: Board<Index<Piece>>,
}

impl<'a> State<'a> {
    pub fn new(rules: &'a Rules) -> Self {
        Self {
            piece_info: PieceInfoTable::init(),
            rules,
            turn: Team::White,
            board: Board::new(),
        }
    }

    pub fn piece_at<T: BoardIndex>(&self, square: T) -> &Option<PieceInfo> {
        &self.piece_info[*self.board[square].get(&*self.rules.pieces) as usize]
    }
    
    pub fn get_piece(&self, idx: Index<Piece>) -> &Option<PieceInfo> {
        &self.piece_info[idx.0 as usize]
    }
}

pub trait StateGet<T> {
    fn get(&self, idx: Index<T>) -> &T;
}