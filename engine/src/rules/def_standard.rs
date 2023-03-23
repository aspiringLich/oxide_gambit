use crate::{
    chess::{direction::Direction, index::Index, square::Square, Team},
    move_gen::{generator::MoveGenerator, normal::NormalMoves},
    state::board_state::BoardState,
};

use super::{
    piece::{Piece, PieceTrait},
    piece_info::PieceInfo,
};

/// If theres a piece on the square, return it and the square
#[inline(always)]
pub fn try_get_square<'a>(
    state: &BoardState,
    pos: Square,
    x: i8,
    y: i8,
) -> Option<(Index<Piece>, Square)> {
    let square = pos.try_move(x, y)?;
    let idx = state.board()[square];
    Some((idx, square))
}

pub struct Invalid;

impl PieceTrait for Invalid {
    fn info(&self) -> PieceInfo {
        panic!("Invalid piece")
    }
}

pub struct Pawn<const T: Team>;

impl<const T: Team> Pawn<T> {
    const DIR: i8 = match T {
        Team::Black => -1,
        Team::White => 1,
    };

    const Y: u8 = match T {
        Team::Black => 6,
        Team::White => 1,
    };
}

impl<const T: Team> PieceTrait for Pawn<T> {
    fn info(&self) -> PieceInfo {
        PieceInfo::new()
            .ch("♟︎")
            .fen_ch('p')
            .name("Pawn")
            .value(1)
            .sprite_index(0)
            .build(T)
    }

    fn moves(&self) -> NormalMoves {
        NormalMoves::new()
            .add_move(0, Self::DIR)
            .add_attack(1, Self::DIR)
            .add_attack(1, Self::DIR)
    }
}

pub struct Rook<const T: Team>;

impl<const T: Team> PieceTrait for Rook<T> {
    fn info(&self) -> PieceInfo {
        PieceInfo::new()
            .ch("♜")
            .fen_ch('r')
            .name("Rook")
            .value(5)
            .attacks(&Direction::ORTHOGONAL)
            .sprite_index(1)
            .build(T)
    }
}

pub struct Knight<const T: Team>;

impl<const T: Team> PieceTrait for Knight<T> {
    fn info(&self) -> PieceInfo {
        PieceInfo::new()
            .ch("♞")
            .fen_ch('n')
            .name("Knight")
            .value(3)
            .sprite_index(2)
            .build(T)
    }
}

pub struct Bishop<const T: Team>;

impl<const T: Team> PieceTrait for Bishop<T> {
    fn info(&self) -> PieceInfo {
        PieceInfo::new()
            .ch("♝")
            .fen_ch('b')
            .name("Bishop")
            .value(3)
            .attacks(&Direction::DIAGONAL)
            .sprite_index(3)
            .build(T)
    }
}

pub struct Queen<const T: Team>;

impl<const T: Team> PieceTrait for Queen<T> {
    fn info(&self) -> PieceInfo {
        PieceInfo::new()
            .ch("♛")
            .fen_ch('q')
            .name("Queen")
            .value(9)
            .attacks(&Direction::ALL)
            .sprite_index(4)
            .build(T)
    }
}

pub struct King<const T: Team>;

impl<const T: Team> PieceTrait for King<T> {
    fn info(&self) -> PieceInfo {
        PieceInfo::new()
            .ch("♚")
            .fen_ch('k')
            .name("King")
            .value(0)
            .sprite_index(5)
            .build(T)
    }
}
