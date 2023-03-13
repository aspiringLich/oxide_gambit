use std::default::default;

use crate::*;

use super::{ square::Square, direction::Direction };

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
    pub fn get<I: BoardIndex>(&self, square: I) -> Option<&T> {
        self.squares.get(square.get())
    }

    pub fn get_mut<I: BoardIndex>(&mut self, square: I) -> Option<&mut T> {
        self.squares.get_mut(square.get())
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
        assert_eq!(board.get(Square(255)), None);
        assert_eq!(board.get(Square(0)), Some(&1));
    }
}