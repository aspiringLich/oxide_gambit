use crate::*;

use super::{direction::Direction, board::Board};

/// Stores the directions that this square is being attacked from by sliding pieces.
/// 
/// Indexed with a [Direction]
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct SlidingAttacks(u8);

impl SlidingAttacks {
    pub fn get(&self, dir: Direction) -> bool {
        self.0 & (1 << dir as u8) != 0
    }
    
    pub fn set(&mut self, dir: Direction, b: bool) {
        self.0 |= (b as u8) << dir as u8;
    }
}

/// Stores the squares that are being attacked by all pieces
/// 
/// Used to determine if a move is legal
pub struct AttackedSquares {
    sliding: Board<SlidingAttacks>,
    non_sliding: Board<u8>
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