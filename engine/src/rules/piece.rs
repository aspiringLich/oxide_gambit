use crate::{
    chess::{direction::Direction, Team},
    move_gen,
};
use derive_more::Deref;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use super::piece_info::PieceInfo;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum Piece {
    #[default]
    Empty,
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

impl Piece {
    pub fn info(&self) -> Option<PieceInfo> {
        use Piece::*;
        if self == &Empty {
            return None;
        }
        Some(match self {
            Empty => unreachable!(),
            WhitePawn => pawn(Team::White),
            WhiteRook => rook(Team::White),
            WhiteKnight => knight(Team::White),
            WhiteBishop => bishop(Team::White),
            WhiteQueen => queen(Team::White),
            WhiteKing => king(Team::White),
            BlackPawn => pawn(Team::Black),
            BlackRook => rook(Team::Black),
            BlackKnight => knight(Team::Black),
            BlackBishop => bishop(Team::Black),
            BlackQueen => queen(Team::Black),
            BlackKing => king(Team::Black),
        })
    }
}

#[derive(Deref, Debug, Clone)]
pub struct PieceInfoTable(pub Vec<Option<PieceInfo>>);

impl PieceInfoTable {
    pub fn init() -> Self {
        Self(Piece::iter().map(|p| p.info()).collect())
    }
}

fn pawn(team: Team) -> PieceInfo {
    PieceInfo::new()
        .ch("♟︎")
        .fen_ch(['p', 'P'][team as usize])
        .name("Pawn")
        .value(1)
        .team(team)
        .move_gen(&move_gen::pawn)
}

fn rook(team: Team) -> PieceInfo {
    PieceInfo::new()
        .ch("♜")
        .fen_ch(['r', 'R'][team as usize])
        .name("Rook")
        .value(5)
        .team(team)
        .attacks(&Direction::ORTHOGONAL)
}

fn knight(team: Team) -> PieceInfo {
    PieceInfo::new()
        .ch("♞")
        .fen_ch(['n', 'N'][team as usize])
        .name("Knight")
        .value(3)
        .team(team)
        .move_gen(&move_gen::knight)
}
fn bishop(team: Team) -> PieceInfo {
    PieceInfo::new()
        .ch("♝")
        .fen_ch(['b', 'B'][team as usize])
        .name("Bishop")
        .value(3)
        .attacks(&Direction::DIAGONAL)
}

fn queen(team: Team) -> PieceInfo {
    PieceInfo::new()
        .ch("♛")
        .fen_ch(['q', 'Q'][team as usize])
        .name("Queen")
        .value(9)
        .attacks(&Direction::ALL)
}

fn king(team: Team) -> PieceInfo {
    PieceInfo::new()
        .ch("♚")
        .fen_ch(['k', 'K'][team as usize])
        .name("King")
        .value(0)
        .move_gen(&move_gen::king)
}
