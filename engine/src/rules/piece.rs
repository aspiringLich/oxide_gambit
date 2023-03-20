use crate::{
    chess::{square::Square, Team},
    move_gen::{generator::MoveGenerator, moves::Moves},
    state::{board_state::BoardState},
};


use strum_macros::EnumIter;

use super::piece_info::PieceInfo;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum Piece {
    #[default]
    Empty,
    Captured,
    WhitePawn,
    WhiteRook,
    WhiteKnight,
    WhiteBishop,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackRook,
    BlackKnight,
    BlackBishop,
    BlackQueen,
    BlackKing,
}

pub trait PieceTrait {
    /// Get information about the piece
    fn info(&self) -> PieceInfo;
    fn move_gen(&self, state: &BoardState, moves: &mut Moves, square: Square) {
        self.move_gen_internal(MoveGenerator::new(moves, state, square));
    }
    /// Generate the moves for the piece initially
    #[allow(unused_variables, unused_mut)]
    #[inline(always)]
    fn move_gen_internal(&self, mut moves: MoveGenerator) {}
}

impl Piece {
    pub fn piece(&self) -> Option<Box<dyn PieceTrait>> {
        use super::def_standard::*;
        use Piece::*;

        fn f<T: PieceTrait + 'static>(p: T) -> Option<Box<dyn PieceTrait>> {
            Some(Box::new(p))
        }

        match self {
            Empty => None,
            Captured => None,
            WhitePawn => f(Pawn::<{ Team::White }>),
            WhiteRook => f(Rook::<{ Team::White }>),
            WhiteKnight => f(Knight::<{ Team::White }>),
            WhiteBishop => f(Bishop::<{ Team::White }>),
            WhiteQueen => f(Queen::<{ Team::White }>),
            WhiteKing => f(King::<{ Team::White }>),
            BlackPawn => f(Pawn::<{ Team::Black }>),
            BlackRook => f(Rook::<{ Team::Black }>),
            BlackKnight => f(Knight::<{ Team::Black }>),
            BlackBishop => f(Bishop::<{ Team::Black }>),
            BlackQueen => f(Queen::<{ Team::Black }>),
            BlackKing => f(King::<{ Team::Black }>),
        }
    }

    pub fn info(self) -> Option<PieceInfo> {
        if let Some(piece) = self.piece() {
            Some(piece.info())
        } else {
            None
        }
    }
}
