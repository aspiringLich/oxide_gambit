use std::{borrow::Borrow, collections::VecDeque};

use super::{ChessState, Piece, PieceType, PieceVariant, Position};
use bevy::prelude::*;

/// struct for holding a chess move
///     start - starting position
///     end - ending position
#[derive(Debug, Copy, Clone)]
pub struct ChessMove {
    pub origin: Position,
    pub target: Position,
}

impl ChessMove {
    pub const fn new(origin: Position, target: Position) -> Self {
        ChessMove { origin, target }
    }
}

enum MovesIndex {
    M_Straight = 0,
    M_Diagonal,
    M_All,
    M_Knight,
}

lazy_static! {
    static ref MOVES: [Vec<(i8, i8)>; 4] = {
        let mut m: [Vec<(i8, i8)>; 4] = default();
        m[0] = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        m[1] = vec![(1, 1), (1, -1), (-1, -1), (-1, 1)];
        m[2] = vec![(0, 1), (1, 0), (0, -1), (-1, 0), (1, 1), (1, -1), (-1, -1), (-1, 1)];
        m[3] = vec![(-1, 2), (1, 2), (2, 1), (2, -1), (1, -2), (-1, -2), (-2, -1), (-2, 1)];
        m
    };
}

impl ChessState {
    pub const fn at(&self, pos: Position) -> PieceType {
        self.board[pos.0 as usize]
    }

    pub fn occupied(&self, pos: Position) -> bool {
        self.at(pos).variant() != PieceVariant::None
    }

    pub const fn team(&self, pos: Position) -> bool {
        self.at(pos).team()
    }

    /// if a square is capturable, check if occupied first
    pub const fn capturable(&self, pos: Position) -> bool {
        (self.team(pos) != self.turn)
    }

    pub const fn variant(&self, pos: Position) -> PieceVariant {
        self.at(pos).variant()
    }
}
impl ChessState {
    /// generate the moves based on the current board state
    pub fn move_gen(&mut self) {
        use MovesIndex::*;
        use PieceVariant::*;

        self.moves.clear();

        for piece in self.pieces[self.turn as usize].clone() {
            match piece.variant() {
                None => panic!("wee woo invalid piece in piece vec or something"),
                // pawn
                Pawn => self.gen_pawn_moves(piece),
                // sliding pieces
                Rook => self.gen_sliding(piece, &MOVES[M_Straight as usize]),
                Bishop => self.gen_sliding(piece, &MOVES[M_Diagonal as usize]),
                Queen => self.gen_sliding(piece, &MOVES[M_All as usize]),
                // "static" pieces
                King => self.gen_static(piece, &MOVES[M_All as usize]),
                Knight => self.gen_static(piece, &MOVES[M_Knight as usize]),
            }
        }
    }

    fn push_move(&mut self, piece: Piece, pos: Position) {
        if self.occupied(pos) {
            if self.capturable(pos) {
                self.moves.push_front(ChessMove::new(piece.position, pos));
            }
        } else {
            self.moves.push_back(ChessMove::new(piece.position, pos));
        }
    }

    /// generate the moves for a static set of movements
    #[inline]
    pub fn gen_static(&mut self, piece: Piece, movements: &Vec<(i8, i8)>) {
        for movement in movements {
            if let Some(pos) = piece.try_to(*movement) {
                self.push_move(piece, pos)
            }
        }
    }

    /// generate moves on a list of directions
    #[inline]
    pub fn gen_sliding(&mut self, piece: Piece, movements: &Vec<(i8, i8)>) {
        for movement in movements {
            self.gen_sliding_dir(piece, *movement);
        }
    }

    /// generate all pieces in a direction
    #[inline]
    pub fn gen_sliding_dir(&mut self, piece: Piece, direction: (i8, i8)) {
        let (x, y) = direction;

        // while we can still move in this direction
        let mut iter = 1;
        while let Some(pos) = piece.try_to((x * iter, y * iter)) {
            // if its occupied oh noes
            self.push_move(piece, pos);
            iter += 1;
        }
    }

    /// generate moves a pawn could take
    /// TODO: add en passant
    #[inline]
    pub fn gen_pawn_moves(&mut self, piece: Piece) {
        let dir = if piece.team() { 1 } else { -1 };

        let double_available = || piece.y() == [6, 1][piece.team() as usize];
        let promotion_available = || piece.y() == [1, 6][piece.team() as usize];

        // forward
        if let Some(pos) = piece.try_to((0, dir)) {
            if !self.occupied(pos) {
                self.moves.push_back(ChessMove::new(piece.position, pos))
            }
        }

        // double forward
        if double_available() {
            if let Some(pos) = piece.try_to((0, dir * 2)) {
                if !self.occupied(pos) {
                    self.moves.push_back(ChessMove::new(piece.position, pos))
                }
            }
        }

        // capture
        let mut capture = |movement: (i8, i8)| {
            if let Some(pos) = piece.try_to(movement) {
                if self.occupied(pos) && self.capturable(pos) {
                    self.moves.push_back(ChessMove::new(piece.position, pos))
                }
            }
        };
        capture((dir, dir));
        capture((-dir, dir));
    }
}
