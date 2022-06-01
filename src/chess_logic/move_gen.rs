use bevy::prelude::default;

use super::{ChessState, Piece, PieceType, PieceVariant, Position};

/// enum for storing the attribute
#[derive(Debug)]
pub enum MoveAttribute {
    None,
    Capture,
    Check,
    Checkmate,
    Stalemate,
    Promotion,
}

/// struct for holding a chess move
///     start - starting position
///     end - ending position
#[derive(Debug)]
pub struct ChessMove {
    pub origin: Position,
    pub target: Position,
    pub attribute: MoveAttribute,
}

impl ChessMove {
    pub const fn new(origin: Position, target: Position, attribute: MoveAttribute) -> Self {
        ChessMove { origin, target, attribute }
    }
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

    pub fn move_gen(&mut self) {
        use PieceVariant::*;
        for piece in self.pieces[self.turn as usize].clone() {
            let mut target = {
                match piece.variant() {
                    None => panic!("wee woo invalid piece in piece vec"),
                    Pawn => self.gen_pawn_moves(piece),
                    Rook => self.gen_sliding(piece, vec![(0, 1), (0, -1), (1, 0), (-1, 0)]),
                    Bishop => self.gen_sliding(piece, vec![(1, 1), (1, -1), (-1, -1), (-1, 1)]),
                    Knight => todo!(),
                    King => todo!(),
                    Queen => self.gen_sliding(
                        piece,
                        vec![(0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1)],
                    ),
                }
            };
            self.moves.append(&mut target);
        }
    }

    /// generate moves on a list of directions
    pub fn gen_sliding(&self, piece: Piece, movements: Vec<(i8, i8)>) -> Vec<ChessMove> {
        let mut out: Vec<ChessMove> = vec![];

        for movement in movements {
            out.append(&mut self.gen_sliding_dir(piece, movement));
        }
        return out;
    }

    /// generate all pieces in a direction
    pub fn gen_sliding_dir(&self, piece: Piece, direction: (i8, i8)) -> Vec<ChessMove> {
        use MoveAttribute::*;

        let mut out: Vec<ChessMove> = vec![];
        let (x, y) = direction;

        // while we can still move in this direction
        let mut iter = 1;
        while let Some(new_pos) = piece.try_to((x * iter, y * iter)) {
            // if its occupied oh noes
            if self.occupied(new_pos) {
                // we can capture a piece?
                if self.capturable(new_pos) {
                    out.push(ChessMove::new(piece.position, new_pos, Capture));
                }
                break;
            }
            // push a new move if empty
            out.push(ChessMove::new(piece.position, new_pos, None));
            iter += 1;
        }
        out
    }

    /// generate moves a pawn could take
    /// TODO: add en passant
    #[inline]
    pub fn gen_pawn_moves(&self, piece: Piece) -> Vec<ChessMove> {
        use MoveAttribute::*;

        let dir = if piece.team() { 1 } else { -1 };
        let mut out: Vec<ChessMove> = vec![];

        let double_available = || piece.y() == [6, 1][piece.team() as usize];
        let promotion_available = || piece.y() == [1, 6][piece.team() as usize];

        let mut push_move =
            |attribute, target| out.push(ChessMove::new(piece.position, target, attribute));
        let target: Position = default();

        // forward
        if let Some(target) = piece.try_to((0, dir)) {
            if !self.occupied(target) {
                push_move(if promotion_available() { Promotion } else { None }, target)
            }
        }

        // double forward
        if double_available() {
            if let Some(target) = piece.try_to((0, dir * 2)) {
                if !self.occupied(target) {
                    push_move(None, target)
                }
            }
        }

        // capture
        let mut capture = |movement: (i8, i8)| {
            if let Some(target) = piece.try_to(movement) {
                if self.occupied(target) && self.capturable(target) {
                    push_move(if promotion_available() { Promotion } else { None }, target)
                }
            }
        };
        capture((dir, dir));
        capture((-dir, dir));

        return out;
    }
}
