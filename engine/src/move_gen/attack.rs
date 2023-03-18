use crate::*;

use crate::chess::square::Square;
use crate::chess::{board::Board, direction::Direction};

/// Stores the directions that this square is being attacked from by sliding pieces.
///
/// Indexed with a [Direction]
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct SlidingAttacks(pub u8);

impl SlidingAttacks {
    pub fn get(&self, dir: Direction) -> bool {
        self.0 & (1 << dir as u8) != 0
    }

    pub fn set(&mut self, dir: Direction, b: bool) {
        debug_assert!(self.0 & (1 << dir as u8) == (!b as u8) << dir as u8);
        self.0 |= (b as u8) << dir as u8;
    }
}

/// Stores the squares that are being attacked by all pieces
///
/// Used to determine if a move is legal
#[derive(Default, Clone, Debug)]
pub struct AttackedSquares {
    pub sliding: Board<SlidingAttacks>,
    pub non_sliding: Board<u8>,
}

impl AttackedSquares {
    /// Signals that another piece is attacking this square
    pub fn inc(&mut self, square: Square) {
        self.non_sliding[square] += 1;
    }
    
    /// Signals that a piece is no longer attacking this square
    pub fn dec(&mut self, square: Square) {
        self.non_sliding[square] -= 1;
    }
    
    /// Signals that a sliding piece is attacking this square
    pub fn add_sliding(&mut self, square: Square, dir: Direction) {
        self.sliding[square].set(dir, true);
    }
    
    /// Signals that a sliding piece is no longer attacking this square
    pub fn remove_sliding(&mut self, square: Square, dir: Direction) {
        self.sliding[square].set(dir, false);
    }
    
    /// Returns true if this square is being attacked by any piece
    pub fn is_attacked(&self, square: Square) -> bool {
        self.non_sliding[square] != 0 || self.sliding[square].0 != 0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_attacking() {
        let mut attacking = SlidingAttacks(0);
        attacking.set(Direction::N, true);
        assert_eq!(attacking.get(Direction::N), true);
        assert_eq!(attacking.0, 0b0000_0100);
    }
}
