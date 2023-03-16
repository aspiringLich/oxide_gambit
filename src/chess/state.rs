use crate::rules::{piece::PieceInfo, Rules};

use super::{
    board::{Board, BoardIndex},
    Team,
};

/// A u8 that serves as an index into a list of `T`
#[derive(Debug, PartialEq, Eq)]
pub struct Index<T>(u8, std::marker::PhantomData<T>);

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
}

/// A struct representing the state of a chess game
#[derive(Clone)]
pub struct State<'a> {
    /// The rules of the game
    pub rules: &'a Rules,
    /// The team whose turn it is
    pub turn: Team,
    /// The board: the pieces are indexes into `rules.piece_info`
    pub board: Board<Index<PieceInfo>>,
}

impl<'a> State<'a> {
    pub fn new(rules: &'a Rules) -> Self {
        Self {
            rules,
            turn: Team::White,
            board: Board::new(),
        }
    }

    pub fn piece_at<T: BoardIndex>(&self, square: T) -> &PieceInfo {
        self.board[square].get(&self.rules.piece_info)
    }
}
