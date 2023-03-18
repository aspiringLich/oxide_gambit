use crate::{
    chess::{direction::Direction, Team, square::Square},
    move_gen::{self, moves::Moves}, state::state::State,
};
use derive_more::Deref;
use strum::IntoEnumIterator;
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

pub trait PieceTrait<const T: Team> {
    /// Get the name of the piece
    fn name(&self) -> &str {
        &self.info().name
    }
    /// Get the character used to represent the piece
    fn ch(&self) -> &'static str {
        self.info().ch
    }
    /// Get the character used to represent the piece in a FEN string
    fn fen_ch(&self) -> Option<char> {
        self.info().fen_ch
    }
    /// Get the team that the piece belongs to
    fn value(&self) -> u8 {
        self.info().value
    }
    /// Get the callback squares of the piece
    fn callbacks(&self) -> &[(i8, i8)] {
        &self.info().callbacks
    }
    /// Get the directions that this piece can attack in
    fn attacks(&self) -> &[Direction] {
        &self.info().attacks
    }
    /// Get information about the piece
    fn info(&self) -> PieceInfo;
    /// Generate the moves for the piece initially
    fn move_gen(&self, state: &State, moves: &mut Moves, square: Square);
}

impl Piece {
    pub fn info(&self) -> Option<PieceInfo> {
        use Piece::*;
        use super::def_standard::*;
        if matches!(self, Empty | Captured) {
            return None;
        }
        Some(match self {
            Empty => unreachable!(),
            Captured => unreachable!(),
            WhitePawn => Pawn::<{Team::White}>.info(),
            WhiteRook => Rook::<{Team::White}>.info(),
            WhiteKnight => Knight::<{Team::White}>.info(),
            WhiteBishop => Bishop::<{Team::White}>.info(),
            WhiteQueen => Queen::<{Team::White}>.info(),
            WhiteKing => King::<{Team::White}>.info(),
            BlackPawn => Pawn::<{Team::Black}>.info(),
            BlackRook => Rook::<{Team::Black}>.info(),
            BlackKnight => Knight::<{Team::Black}>.info(),
            BlackBishop => Bishop::<{Team::Black}>.info(),
            BlackQueen => Queen::<{Team::Black}>.info(),
            BlackKing => King::<{Team::Black}>.info(),
        })
    }
}
