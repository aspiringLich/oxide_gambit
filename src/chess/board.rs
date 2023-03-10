use std::default::default;

use crate::*;

use super::square::Square;

pub trait BoardType = Default + Copy;

#[derive(Deref, DerefMut, Clone, Debug)]
pub struct Board<T: BoardType> {
    squares: [T; 64]
}

impl<T: BoardType> Board<T> {
    pub fn new() -> Board<T> {
        Board { squares: [default(); 64] }
    }
}

impl<T: BoardType> std::ops::Index<Square> for Board<T> {
    type Output = T;
    fn index(&self, index: Square) -> &Self::Output {
        &self.squares[*index as usize]
    }
}

impl <T: BoardType> std::ops::IndexMut<Square> for Board<T> {
    fn index_mut(&mut self, index: Square) -> &mut Self::Output {
        &mut self.squares[*index as usize]
    }
}

