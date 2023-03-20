use crate::{
    chess::{direction::Direction, index::Index, square::Square, Team},
    move_gen::{generator::MoveGenerator, moves::Moves},
    state::{board_state::BoardState, state::State},
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

    fn move_gen_internal(&self, moves: MoveGenerator) {
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
            .callbacks(&[
                (0, Self::DIR),
                (0, Self::DIR * 2),
                (1, Self::DIR),
                (-1, Self::DIR),
            ])
            .sprite_index(0)
            .build(T)
    }

    fn move_gen_internal(&self, mut moves: MoveGenerator) {
        // forward && forward * 2
        if let Some(square) = moves.try_get_empty(0, Self::DIR) {
            moves.insert(square);
            if moves.square.y() == Self::Y && let Some(square) = moves.try_get_empty(0, Self::DIR * 2) {
                moves.insert(square);
            }

            // capture
            if let Some((square, Some(piece))) = moves.try_get_square(1, Self::DIR) && piece.team != T {
                moves.insert(square);
            }
            if let Some((square, Some(piece))) = moves.try_get_square(-1, Self::DIR) && piece.team != T {
                moves.insert(square);
            }
        }
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

impl<const T: Team> Knight<T> {
    const MOVES: [(i8, i8); 8] = [
        (1, 2),
        (2, 1),
        (2, -1),
        (1, -2),
        (-1, -2),
        (-2, -1),
        (-2, 1),
        (-1, 2),
    ];
}

impl<const T: Team> PieceTrait for Knight<T> {
    fn info(&self) -> PieceInfo {
        PieceInfo::new()
            .ch("♞")
            .fen_ch('n')
            .name("Knight")
            .value(3)
            .callbacks(&Self::MOVES)
            .sprite_index(2)
            .build(T)
    }

    fn move_gen_internal(&self, mut moves: MoveGenerator) {
        for &(x, y) in Self::MOVES.iter() {
            moves.try_capture(x, y, T);
        }
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

impl<const T: Team> King<T> {
    const MOVES: [(i8, i8); 8] = [
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];
}

impl<const T: Team> PieceTrait for King<T> {
    fn info(&self) -> PieceInfo {
        PieceInfo::new()
            .ch("♚")
            .fen_ch('k')
            .name("King")
            .value(0)
            .callbacks(&Self::MOVES)
            .sprite_index(5)
            .build(T)
    }

    fn move_gen_internal(&self, mut moves: MoveGenerator) {
        for &(x, y) in Self::MOVES.iter() {
            moves.try_capture(x, y, T)
        }
    }
}
