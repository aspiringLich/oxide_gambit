use std::default::default;

use crate::*;

use super::{ square::Square, direction::Direction };

pub trait BoardType = Default + Copy;

#[derive(Deref, DerefMut, Clone, Debug)]
pub struct Board<T: BoardType> {
    pub squares: [T; 64],
}

impl<T: BoardType> Board<T> {
    pub fn new() -> Board<T> {
        Board { squares: [default(); 64] }
    }
}

pub struct DirectionIterator<'a, T: BoardType> {
    board: &'a Board<T>,
    dx: i8,
    dy: i8,
    x: u8,
    y: u8,
}

impl<'a, T: BoardType> Iterator for DirectionIterator<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let x = (self.x as i8) + self.dx;
        (x >= 0 && x < 8).then(|| {
            self.x = x as u8;
        });
        let y = (self.y as i8) + self.dy;
        (y >= 0 && y < 8).then(|| { 
            self.y = y as u8;
        });

        Some(self.board[(y * 8 + x) as usize])
    }
}

impl<T: BoardType> Board<T> {
    pub fn get<I: BoardIndex>(&self, square: I) -> Option<&T> {
        self.squares.get(square.get())
    }

    pub fn get_mut<I: BoardIndex>(&mut self, square: I) -> Option<&mut T> {
        self.squares.get_mut(square.get())
    }

    pub fn iter_direction<'a, I: BoardIndex>(&'a self, dir: Direction, start: I) -> DirectionIterator<'a, T> {
        DirectionIterator { board: self, dx: dir.x(), dy: dir.y(), x: (start.get() % 8) as u8, y: (start.get() / 8) as u8 }
    }
}

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

    #[test]
    fn test_direction_iter() {
        let mut board: Board<i32> = Board::new();
        let mut i = 0;
        board.squares = board.squares.map(|_| {
            i += 1;
            i - 1
        });
        
        assert_eq!(board.iter_direction(Direction::NE, 0).collect::<Vec<_>>(), vec![0, 9, 18, 27, 36, 45, 54, 63]);
    }
}