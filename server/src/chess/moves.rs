use core::slice::Iter;
use std::{
    collections::BTreeMap,
    iter::{Chain, Map},
    ops::{Deref, DerefMut, Index},
};

use super::{pieces::Piece, square::Square};

/// a structure for holding the squares a piece can move to
///  - key: the chess move
///  -
#[derive(Default, Clone, Debug)]
pub struct Moves {
    pub priority: Vec<Square>,
    pub other: Vec<Square>,
}

pub enum Priority {
    High,
    Low,
}

impl Moves {
    /// returns which priority contains the square if any
    /// else returns None
    pub fn contains_move(&mut self, square: Square) -> Option<(&mut Square, Priority)> {
        // if the move is in priority return it
        if let Some(square) = self.priority.iter_mut().find(|x| **x == square) {
            return Some((square, Priority::High));
        }
        // youll never guess what this does
        if let Some(square) = self.other.iter_mut().find(|x| **x == square) {
            return Some((square, Priority::Low));
        }
        // else its not here bub
        None
    }

    /// adds a move, and
    pub fn add_move(&mut self, square: Square, priority: Priority) {
        let ret = self.contains_move(square);
        match ret {
            Some(_) => (),
            None => match priority {
                Priority::High => self.priority.push(square),
                Priority::Low => self.other.push(square),
            },
        }
    }
}
