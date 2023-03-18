use std::default::default;

use rustc_hash::{FxHashMap, FxHashSet};

use crate::chess::board::Board;
use crate::chess::direction::Direction;
use crate::chess::index::Index;
use crate::chess::Team;
use crate::rules::piece::Piece;

use crate::chess::square::Square;
use crate::state::board_state::BoardState;

use super::attack::AttackedSquares;

/// A move from one square to another
#[derive(Hash, PartialEq, Eq, Debug)]
pub struct Move {
    pub piece: Index<Piece>,
    pub to: Square,
    // pub active: bool,
}

/// Stores the list of moves that can be made
#[derive(Debug)]
pub struct Moves {
    moves: FxHashSet<Move>,
    callbacks: FxHashMap<Square, Vec<Index<Piece>>>,
    attacked: AttackedSquares,
}

impl Moves {
    pub fn new() -> Self {
        Self {
            moves: default(),
            callbacks: default(),
            attacked: default(),
        }
    }

    /// Inserts a sliding move into the list of moves
    ///
    /// # Arguments
    ///  - `team`: The team of the piece that is moving
    ///  - `square`: The square that the piece is moving from
    ///  - `dir`: The direction that the piece is moving in
    ///  - `board`: The board state
    ///  - `active`: Whether or not the move is active
    ///  - `inclusive`: Whether or not to include the starting square
    pub fn insert_sliding(
        &mut self,
        team: Team,
        square: Square,
        dir: Direction,
        board: &BoardState,
        inclusive: bool,
    ) {
        // go through the squares in this direction
        for &idx in board
            .board()
            .iter_direction(dir, square)
            .skip((!inclusive) as usize)
        {
            if let Some(piece) = board.get_idx(idx) {
                // if the piece is on the other team, add it to the list of moves
                if piece.team != team {
                    self.insert_good(idx, square);
                    self.attacked.add_sliding(square, dir);
                }
                break;
            } else {
                self.insert(idx, square);
                self.attacked.add_sliding(square, dir);
            }
        }
    }
    
    /// Removes a sliding move from the list of moves
    ///
    /// # Arguments
    /// - `team`: The team of the piece that is moving
    /// - `square`: The square that the piece is moving from
    /// - `dir`: The direction that the piece is moving in
    /// - `board`: The board state
    /// - `inclusive`: Whether or not to include the starting square
    pub fn remove_sliding(
        &mut self,
        team: Team,
        square: Square,
        dir: Direction,
        board: &BoardState,
        inclusive: bool,
    ) {
        // go through the squares in this direction
        for &idx in board
            .board()
            .iter_direction(dir, square)
            .skip((!inclusive) as usize)
        {
            if let Some(piece) = board.get_idx(idx) {
                // if the piece is on the other team, remove it too
                if piece.team != team {
                    debug_assert!(self.remove(idx, square), "Expected move to exist {:?} -> {:?}", idx, square);
                    self.attacked.remove_sliding(square, dir);
                }
                break;
            } else {
                self.remove(idx, square);
                self.attacked.remove_sliding(square, dir);
            }
        }
    }

    /// Inserts a move into the list of moves
    pub fn insert(&mut self, piece: Index<Piece>, to: Square) {
        let check = self.moves.insert(Move {
            piece,
            to,
        });
        debug_assert!(!check, "Move already exists: {:?} -> {:?}", piece, to);
        self.attacked.inc(to);
    }
    
    /// Removes a move from the list of moves, returns whether the move was there to begin with
    pub fn remove(&mut self, piece: Index<Piece>, to: Square) -> bool {
        self.attacked.dec(to);
       self.moves.remove(&Move {
            piece,
            to,
        })
    }

    /// Inserts an inactive move into the list of moves
    pub fn insert_inactive(&mut self, piece: Index<Piece>, to: Square) {
        // self.moves.insert(Move {
        //     piece,
        //     to,
        // });
        match self.callbacks.get_mut(&to) {
            Some(callbacks) => callbacks.push(piece),
            None => {
                self.callbacks.insert(to, vec![piece]);
            }
        }
    }

    /// Inserts a *good* move into the list of moves
    pub fn insert_good(&mut self, piece: Index<Piece>, to: Square) {
        self.moves.insert(Move {
            piece,
            to,
            // active: true,
        });
        self.attacked.inc(to);
    }
}
