use crate::{
    chess::{square::Square, Team, direction::Direction},
    move_gen::moves::Moves,
    state::state::State,
};

use super::{piece::PieceTrait, piece_info::PieceInfo};

pub struct Pawn<const T: Team>;

impl<const T: Team> PieceTrait<T> for Pawn<T> {
    fn info(&self) -> PieceInfo {
        PieceInfo::new()
            .ch("♟︎")
            .fen_ch('p')
            .name("Pawn")
            .value(1)
            .build(T)
    }

    fn move_gen(&self, state: &State, moves: &mut Moves, square: Square) {}
}

pub struct Rook<const T: Team>;

impl<const T: Team> PieceTrait<T> for Rook<T> {
    fn info(&self) -> PieceInfo {
        PieceInfo::new()
        .ch("♜")
            .fen_ch('r')
            .name("Rook")
            .value(5)
            .attacks(&Direction::ORTHOGONAL)
            .build(T)
    }

    fn move_gen(&self, state: &State, moves: &mut Moves, square: Square) {}
}

pub struct Knight<const T: Team>;

impl<const T: Team> PieceTrait<T> for Knight<T> {
    fn info(&self) -> PieceInfo {
        PieceInfo::new()
            .ch("♞")
            .fen_ch('n')
            .name("Knight")
            .value(3)
            .callbacks(&[(1, 2), (2, 1), (2, -1), (1, -2), (-1, -2), (-2, -1), (-2, 1), (-1, 2)])
            .build(T)
    }

    fn move_gen(&self, state: &State, moves: &mut Moves, square: Square) {}
}

pub struct Bishop<const T: Team>;

impl<const T: Team> PieceTrait<T> for Bishop<T> {
    fn info(&self) -> PieceInfo {
        PieceInfo::new()
            .ch("♝")
            .fen_ch('b')
            .name("Bishop")
            .value(3)
            .attacks(&Direction::DIAGONAL)
            .build(T)
    }

    fn move_gen(&self, state: &State, moves: &mut Moves, square: Square) {}
}

pub struct Queen<const T: Team>;

impl<const T: Team> PieceTrait<T> for Queen<T> {
    fn info(&self) -> PieceInfo {
        PieceInfo::new()
            .ch("♛")
            .fen_ch('q')
            .name("Queen")
            .value(9)
            .attacks(&Direction::ORTHOGONAL)
            .attacks(&Direction::DIAGONAL)
            .build(T)
    }

    fn move_gen(&self, state: &State, moves: &mut Moves, square: Square) {}
}

pub struct King<const T: Team>;

impl<const T: Team> PieceTrait<T> for King<T> {
    fn info(&self) -> PieceInfo {
        PieceInfo::new()
            .ch("♚")
            .fen_ch('k')
            .name("King")
            .value(0)
            .callbacks(&[(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)])
            .build(T)
    }

    fn move_gen(&self, state: &State, moves: &mut Moves, square: Square) {}
}