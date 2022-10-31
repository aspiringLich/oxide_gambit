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

// pub struct MovesIntoIterator {
//     moves: Moves,
//     index: usize,
// }

impl Moves {
    pub fn add_move(&mut self, piece: Piece, priority: u8) {}
}
