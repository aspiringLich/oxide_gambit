use derive_more::{Deref, DerefMut};
use strum_macros::Display;

use crate::{
    chess::{index::Index, square::Square, Team},
    rules::piece::Piece,
    state::{
        board_state::{BoardState, PIECES},
        State,
    },
};

use super::moves::Moves;

/// A relative position on the board.
#[derive(Debug, Copy, Clone)]
pub struct RelativePos {
    pub x: i8,
    pub y: i8,
}

impl RelativePos {
    /// Create a new RelativePos from x and y coordinates.
    pub fn new(x: i8, y: i8) -> Self {
        Self { x, y }
    }

    /// Add a RelativePos to a Square.
    pub fn try_add(self, square: Square) -> Option<Square> {
        let x = square.x() as i8 + self.x;
        let y = square.y() as i8 + self.y;
        Square::from_xy(x, y)
    }
}

#[derive(Debug, Copy, Clone, Display)]
pub enum NormalMoveType {
    None,
    Attack,
    Move,
    Normal,
}

/// A relative move a piece can make
#[derive(Debug, Clone)]
pub struct RelativeMove {
    pub pos: RelativePos,
    pub move_type: NormalMoveType,
}

/// Stores a list of moves a piece can make
#[derive(Deref, DerefMut, Debug, Default)]
pub struct NormalMoves {
    pub moves: Vec<RelativeMove>,
}

impl NormalMoves {
    pub fn new() -> Self {
        Self { moves: Vec::new() }
    }

    /// Add a move to the list of moves
    pub fn add(mut self, x: i8, y: i8) -> Self {
        self.moves.push(RelativeMove {
            pos: RelativePos::new(x, y),
            move_type: NormalMoveType::Normal,
        });
        self
    }

    /// Add an attack to the list of moves
    pub fn add_attack(mut self, x: i8, y: i8) -> Self {
        self.moves.push(RelativeMove {
            pos: RelativePos::new(x, y),
            move_type: NormalMoveType::Attack,
        });
        self
    }

    /// Add a non_attacking move to the list of moves
    pub fn add_move(mut self, x: i8, y: i8) -> Self {
        self.moves.push(RelativeMove {
            pos: RelativePos::new(x, y),
            move_type: NormalMoveType::Move,
        });
        self
    }
}

static mut MOVES: Vec<NormalMoves> = vec![];

pub fn init() {
    unsafe {
        PIECES.iter().for_each(|i| {
            MOVES.push(i.moves());
        });
    }
}

impl Moves {
    /// Add the normal moves for a piece to a list of moves
    pub fn add_normal_moves(
        &mut self,
        board: &BoardState,
        idx: Index<Piece>,
        square: Square,
        team: Team,
    ) {
        let moves = unsafe { &MOVES[*idx.get(board.pieces()) as usize] };
        for relative in moves.iter() {
            // try and get the square
            let Some(square) = relative.pos.try_add(square) else { return };

            use NormalMoveType::*;
            let capture = matches!(relative.move_type, Attack | Normal);
            let mut _move = matches!(relative.move_type, Move | Normal);

            let info = board.get_info(board.board()[square]);

            // if the move is a capture, check if the square is occupied
            if capture {
                // insert the move, and we dont need the move anymore
                if let Some(info) = info && info.team != team {
                    self.insert_good(idx, square, team);
                    self.insert_threat(idx, square, team);
                    self.insert_callback(square, idx);
                    _move = false;
                }
                // if not, add the threat
                else {
                    self.insert_threat(idx, square, team);
                    self.insert_callback(square, idx);
                }
            }
            if _move {
                if info.is_none() {
                    self.insert(idx, square, team);
                    self.insert_callback(square, idx);
                }
            }
        }
    }

    /// Remove the normal moves for a piece from a list of moves
    pub fn remove_normal_moves(
        &mut self,
        board: &BoardState,
        idx: Index<Piece>,
        square: Square,
        team: Team,
    ) {
        let moves = unsafe { &MOVES[*idx.get(board.pieces()) as usize] };
        for relative in moves.iter() {
            // try and get the square
            let Some(square) = relative.pos.try_add(square) else { return };

            use NormalMoveType::*;
            let capture = matches!(relative.move_type, Attack | Normal);
            let mut _move = matches!(relative.move_type, Move | Normal);

            let info = board.get_info(board.board()[square]);

            // if the move is a capture, check if the square is occupied
            if capture {
                // remove the move
                if let Some(info) = info && info.team != team {
                    debug_assert!(self.remove_good(idx, square, team));
                    self.remove_threat(idx, square, team);
                    self.remove_callback(square, idx);
                    _move = false;
                }
                // if not, remove the threat
                else {
                    self.remove_threat(idx, square, team);
                    self.remove_callback(square, idx);
                }
            }
            if _move {
                if info.is_none() {
                    debug_assert!(self.remove(idx, square, team));
                    self.remove_callback(square, idx);
                }
            }
        }
    }
}
