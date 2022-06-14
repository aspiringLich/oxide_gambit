use std::{borrow::Borrow, collections::VecDeque};

use crate::chess_logic::pin::PinType;

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

#[derive(Debug, Clone, Copy)]
pub enum Moves {
    RookMoves = 0,
    BishopMoves,
    QueenMoves,
    KnightMoves,
    PawnWMoves,
    PawnBMoves,
}
impl Moves {
    pub fn get(self) -> &'static Vec<(i8, i8)> {
        &MOVES[self as usize]
    }
}

lazy_static! {
    static ref MOVES: [Vec<(i8, i8)>; 6] = {
        let mut m: [Vec<(i8, i8)>; 6] = default();
        m[0] = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        m[1] = vec![(1, 1), (1, -1), (-1, -1), (-1, 1)];
        m[2] = vec![(0, 1), (1, 0), (0, -1), (-1, 0), (1, 1), (1, -1), (-1, -1), (-1, 1)];
        m[3] = vec![(-1, 2), (1, 2), (2, 1), (2, -1), (1, -2), (-1, -2), (-2, -1), (-2, 1)];
        m[4] = vec![(1, 1), (-1, 1)];
        m[5] = vec![(1, -1), (-1, -1)];
        m
    };
}

impl ChessState {
    pub fn at(&self, pos: Position) -> PieceType {
        self.board[pos.0 as usize]
    }

    pub fn occupied(&self, pos: Position) -> bool {
        self.at(pos).variant() != PieceVariant::None
    }

    pub fn team(&self, pos: Position) -> bool {
        self.at(pos).team()
    }

    /// if a square is capturable, check if occupied first
    pub fn capturable(&self, pos: Position) -> bool {
        (self.team(pos) != self.turn)
    }

    /// get the variant of the piece at this position
    pub fn variant(&self, pos: Position) -> PieceVariant {
        self.at(pos).variant()
    }

    /// add a move to the front (this is a good move)
    #[inline]
    pub fn add_move_front(
        &mut self,
        piece: Piece,
        target: Position,
        direction: (i8, i8),
        index: usize,
    ) {
        use PieceVariant::*;
        use PinType::*;

        // if we have a constraint on the squares to move to
        if let Some(limit) = &self.constraint {
            if limit.binary_search(&target).is_err() && piece.variant() != King {
                return;
            }
        }
        // if the pin direction is not where were trying to move return
        if let Pinned(dir) = self.pinned_pieces[index] {
            eprintln!("pinned");
            if direction != dir && direction != (-dir.0, -dir.1) {
                return;
            }
        }
        self.moves.push_front(ChessMove::new(piece.position, target));
    }

    /// add a move to the back (this is an ok move)
    #[inline]
    pub fn add_move_back(
        &mut self,
        piece: Piece,
        target: Position,
        direction: (i8, i8),
        index: usize,
    ) {
        use PieceVariant::*;
        use PinType::*;

        // if we have a constraint on the squares to move to
        if let Some(limit) = &self.constraint {
            if limit.binary_search(&target).is_err() && piece.variant() != King {
                return;
            }
        }
        // if the pin direction is not where were trying to move return
        if let Pinned(dir) = self.pinned_pieces[index] {
            if !(direction == dir || direction == (-dir.0, -dir.1)) {
                return;
            }
        }
        self.moves.push_back(ChessMove::new(piece.position, target));
    }

    /// generate the moves based on the current board state
    pub fn move_gen(&mut self) {
        use Moves::*;
        use PieceVariant::*;

        self.moves.clear();

        let pieces: &Vec<Piece> = unsafe { std::mem::transmute(&self.pieces[self.turn as usize]) };

        for (i, &piece) in pieces.iter().enumerate() {
            //dbg!(&self.pinned_pieces[i]);
            match piece.variant() {
                None => panic!("wee woo invalid piece in piece vec or something"),
                // pawn
                Pawn => self.gen_pawn_moves(piece, i),
                // sliding pieces
                Rook => self.gen_sliding(piece, RookMoves.get(), i),
                Bishop => self.gen_sliding(piece, BishopMoves.get(), i),
                Queen => self.gen_sliding(piece, QueenMoves.get(), i),
                // "static" pieces
                King => self.gen_static(piece, QueenMoves.get(), i),
                Knight => self.gen_static(piece, KnightMoves.get(), i),
            }
        }

        self.gen_castling();
        dbg!(self);
    }

    /// generate the moves for a static set of movements
    #[inline]
    pub fn gen_static(&mut self, piece: Piece, movements: &Vec<(i8, i8)>, index: usize) {
        for movement in movements {
            if let Some(pos) = piece.try_to(*movement) {
                // you cant move into threatenned squares if king
                if piece.variant() == PieceVariant::King
                    && self.threatened[self.opp_turn()].squares[pos.int()] > 0
                {
                    continue;
                }
                if self.occupied(pos) {
                    if self.capturable(pos) {
                        self.add_move_front(piece, pos, *movement, index);
                    }
                } else {
                    self.add_move_back(piece, pos, *movement, index);
                }
            }
        }
    }

    /// generate moves on a list of directions
    #[inline]
    pub fn gen_sliding(&mut self, piece: Piece, movements: &Vec<(i8, i8)>, index: usize) {
        for movement in movements {
            self.gen_sliding_dir(piece, *movement, index);
        }
    }

    /// generate all pieces in a direction
    #[inline]
    pub fn gen_sliding_dir(&mut self, piece: Piece, direction: (i8, i8), index: usize) {
        let (x, y) = direction;

        // while we can still move in this direction
        let mut iter = 1;
        while let Some(pos) = piece.try_to((x * iter, y * iter)) {
            // if its occupied oh noes
            if self.occupied(pos) {
                if self.capturable(pos) {
                    self.add_move_front(piece, pos, direction, index);
                }
                return;
            } else {
                self.add_move_back(piece, pos, direction, index);
            }
            iter += 1;
        }
    }

    /// generate moves a pawn could take
    /// TODO: add en passant
    #[inline]
    pub fn gen_pawn_moves(&mut self, piece: Piece, index: usize) {
        let dir = if piece.team() { 1 } else { -1 };

        let double_available = || piece.y() == [6, 1][piece.team() as usize];
        let promotion_available = || piece.y() == [1, 6][piece.team() as usize];

        // forward
        if let Some(pos) = piece.try_to((0, dir)) {
            if !self.occupied(pos) {
                if promotion_available() {
                    self.add_move_front(piece, pos, (0, dir), index);
                } else {
                    self.add_move_back(piece, pos, (0, dir), index);
                }

                // double forward
                if double_available() {
                    if let Some(pos) = piece.try_to((0, dir * 2)) {
                        if !self.occupied(pos) {
                            self.add_move_back(piece, pos, (0, dir * 2), index);
                        }
                    }
                }
            }
        }

        // capture
        let mut capture = |movement: (i8, i8)| {
            if let Some(pos) = piece.try_to(movement) {
                if self.occupied(pos) && self.capturable(pos) {
                    self.add_move_back(piece, pos, movement, index);
                }
            }
        };
        capture((dir, dir));
        capture((-dir, dir));
    }

    /// generate all pieces in a direction
    #[inline]
    pub fn gen_sliding_dir_pos(&self, piece: Piece, direction: (i8, i8)) -> Option<Vec<Position>> {
        use crate::PieceVariant::*;
        let mut out: Vec<Position> = vec![];
        let (x, y) = direction;

        // while we can still move in this direction
        let mut iter = 1;
        while let Some(pos) = piece.try_to((x * iter, y * iter)) {
            // if its occupied oh noes
            if self.occupied(pos) {
                if self.at(pos).variant() != King {
                    return Option::None;
                }
                out.push(pos);
                return Some(out);
            } else {
                out.push(pos);
            }
            iter += 1;
        }
        Some(out)
    }
}
