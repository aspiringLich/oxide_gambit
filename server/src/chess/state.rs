use super::{moves::Moves, pieces::Piece, square::Square};
use core::fmt::Debug;
use yauc::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Derivative, FromPrimitive)]
#[repr(u8)]
#[derivative(Default)]
pub enum Team {
    #[derivative(Default)]
    #[default]
    Black,
    White,
}

/// Stores the state of the chess board at any one time
#[derive(Derivative)]
#[derivative(Default)]
pub struct State {
    /// stores the position of all the pieces
    #[derivative(Default(value = "[None; 64]"))]
    board: [Option<Piece>; 64],
    // stores the moves for each piece
    moves: Vec<Box<Moves>>,
    // stores the team that is currently moving (white or black)
    pub turn: Team,
}

impl State {
    /// get the piece at a square
    pub fn get_board<T>(&self, index: T) -> &Option<Piece>
    where
        usize: From<T>,
    {
        &self.board[usize::from(index)]
    }

    /// get the piece at a square but mutable
    pub fn get_board_mut<T>(&mut self, index: T) -> &mut Option<Piece>
    where
        usize: From<T>,
    {
        &mut self.board[usize::from(index)]
    }

    /// adds a piece to the board
    pub fn add_piece_char<T>(&mut self, ch: char, pos: T, id: u8)
    where
        usize: From<T>,
    {
        *self.get_board_mut(pos) = Piece::from_char(ch, id).ok();
    }
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.debug_struct("State")
        //     .field("board", &self.board)
        //     .field("moves", &self.moves)
        //     .field("start_index", &self.start_index)
        //     .finish()
        todo!()
    }
}
