use crate::{
    chess::{direction::Direction, square::Square, Team},
    move_gen::moves::Moves,
    state::{board_state::BoardState, state::State},
};

use super::{piece::PieceTrait, piece_info::PieceInfo};

pub struct Invalid;

impl PieceTrait for Invalid {
    fn info(&self) -> PieceInfo {
        panic!("Invalid piece")
    }

    fn move_gen(&self, state: &BoardState, moves: &mut Moves, square: Square) {
        panic!("Invalid piece")
    }
}

pub struct Pawn<const T: Team>;

impl<const T: Team> PieceTrait for Pawn<T> {
    fn info(&self) -> PieceInfo {
        PieceInfo::new()
            .ch("♟︎")
            .fen_ch('p')
            .name("Pawn")
            .value(1)
            .build(T)
    }

    fn move_gen(&self, state: &BoardState, moves: &mut Moves, square: Square) {}
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
            .build(T)
    }

    fn move_gen(&self, state: &BoardState, moves: &mut Moves, square: Square) {}
}

pub struct Knight<const T: Team>;

impl<const T: Team> PieceTrait for Knight<T> {
    fn info(&self) -> PieceInfo {
        PieceInfo::new()
            .ch("♞")
            .fen_ch('n')
            .name("Knight")
            .value(3)
            .callbacks(&[
                (1, 2),
                (2, 1),
                (2, -1),
                (1, -2),
                (-1, -2),
                (-2, -1),
                (-2, 1),
                (-1, 2),
            ])
            .build(T)
    }

    fn move_gen(&self, state: &BoardState, moves: &mut Moves, square: Square) {}
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
            .build(T)
    }

    fn move_gen(&self, state: &BoardState, moves: &mut Moves, square: Square) {}
}

pub struct Queen<const T: Team>;

impl<const T: Team> PieceTrait for Queen<T> {
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

    fn move_gen(&self, state: &BoardState, moves: &mut Moves, square: Square) {}
}

pub struct King<const T: Team>;

impl<const T: Team> PieceTrait for King<T> {
    fn info(&self) -> PieceInfo {
        PieceInfo::new()
            .ch("♚")
            .fen_ch('k')
            .name("King")
            .value(0)
            .callbacks(&[
                (1, 0),
                (1, 1),
                (0, 1),
                (-1, 1),
                (-1, 0),
                (-1, -1),
                (0, -1),
                (1, -1),
            ])
            .build(T)
    }

    fn move_gen(&self, state: &BoardState, moves: &mut Moves, square: Square) {}
}
