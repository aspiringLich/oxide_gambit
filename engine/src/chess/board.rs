use std::default::default;

use crate::*;

use super::square::Square;

pub trait BoardType = Copy;

#[derive(Deref, DerefMut, Clone, Debug)]
pub struct Board<T: BoardType> {
    pub squares: [T; 64],
}

impl<T: BoardType> Board<T> {
    pub fn new() -> Board<T> where T: Default {
        Board { squares: [default(); 64] }
    }
}

impl<T: BoardType> Board<T> {
    /// Returns the value at the given square if the square is valid
    pub fn get_move(&self, square: Square, x: i8, y: i8) -> Option<&T> {
        let (_x, _y) = square.to_xy();
        let (x, y) = (_x as i8 + x, _y as i8 + y);
        if let Some(square) = Square::from_xy(x, y) {
            Some(&self[square])
        } else {
            None
        }
    }
}

/// Types that can be used to index a board
pub trait BoardIndex {
    fn get(&self) -> usize;
}

impl BoardIndex for usize {
    fn get(&self) -> usize {
        *self
    }
}

impl BoardIndex for Square {
    fn get(&self) -> usize {
        **self as usize
    }
}

impl<T: BoardType, I: BoardIndex> std::ops::Index<I> for Board<T> {
    type Output = T;
    fn index(&self, index: I) -> &Self::Output {
        &self.squares[index.get()]
    }
}

impl<T: BoardType, I: BoardIndex> std::ops::IndexMut<I> for Board<T> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.squares[index.get()]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_board() {
        let mut board = Board::new();
        board[Square(0)] = 1;
        assert_eq!(board[Square(0)], 1);
        assert_eq!(board[Square(1)], 0);
    }
}