use std::ops::Deref;

use crate::{
    chess::{index::Index, square::Square, Team},
    rules::{piece::Piece, piece_info::PieceInfo},
    state::board_state::BoardState,
};

use super::moves::Moves;

pub struct MoveGenerator<'a> {
    pub moves: &'a mut Moves,
    pub state: &'a BoardState,
    pub square: Square,
    pub piece: Index<Piece>,
}

impl<'a> MoveGenerator<'a> {
    pub fn new(moves: &'a mut Moves, state: &'a BoardState, square: Square) -> Self {
        Self {
            moves,
            state,
            square,
            piece: state.board()[square],
        }
    }

    /// If theres a piece on the square, return it and the square
    #[inline(always)]
    pub fn try_get_square(&self, x: i8, y: i8) -> Option<(Square, Option<&PieceInfo>)> {
        let square = self.square.try_move(x, y)?;
        let idx = self.state.board()[square];
        let info = self.state.get_info(idx);
        Some((square, info))
    }

    /// try and get an empty square
    #[inline(always)]
    pub fn try_get_empty(&self, x: i8, y: i8) -> Option<Square> {
        let (square, info) = self.try_get_square(x, y)?;
        if info.is_none() {
            Some(square)
        } else {
            None
        }
    }

    /// Try and add a capture
    #[inline(always)]
    pub fn try_capture(&mut self, x: i8, y: i8, team: Team) {
        if let Some((square, piece)) = self.try_get_square(x, y) {
            // if theres a piece on the square, check if its an enemy
            if let Some(piece) = piece {
                if piece.team != team {
                    self.insert_good(square);
                }
            }
            // else its an empty square
            else {
                self.insert(square);
            }
        }
    }

    /// add a move to the list of moves
    pub fn insert(&mut self, square: Square) {
        self.moves.insert(self.piece, square);
    }

    /// add a *good* move to the list of moves
    pub fn insert_good(&mut self, square: Square) {
        self.moves.insert_good(self.piece, square);
    }
}
