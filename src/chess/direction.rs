

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    E,
    NE,
    N,
    NW,
    W,
    SW,
    S,
    SE
}

use Direction::*;

use super::board::{Board, self};
impl Direction {
    const ORTHOGONAL: [Direction; 4] = [E, N, W, S];
    const DIAGONAL: [Direction; 4] = [NE, NW, SW, SE];
    const ALL: [Direction; 8] = [E, NE, N, NW, W, SW, S, SE];
    
    pub fn xy(self) -> (i8, i8) {
        match self {
            E => (1, 0),
            NE => (1, 1),
            N => (0, 1),
            NW => (-1, 1),
            W => (-1, 0),
            SW => (-1, -1),
            S => (0, -1),
            SE => (1, -1)
        }
    }
    
    pub fn x(self) -> i8 {
        self.xy().0
    }
    
    pub fn y(self) -> i8 {
        self.xy().1
    }
}

pub struct DirectionIterator<'a, T: board::BoardType> {
    board: &'a Board<T>,
    dx: i8,
    dy: i8,
    x: i8,
    y: i8,
}

impl<'a, T: board::BoardType> Iterator for DirectionIterator<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.x < 0 || self.x >= 8 || self.y < 0 || self.y >= 8 {
            return None;
        }
        let out = Some(self.board[(self.y * 8 + self.x) as usize]);
        self.x += self.dx;
        self.y += self.dy;
        out
    }
}

impl<T: board::BoardType> Board<T> {
    pub fn iter_direction<'a, I: board::BoardIndex>(&'a self, dir: Direction, start: I) -> DirectionIterator<'a, T> {
        DirectionIterator { board: self, dx: dir.x(), dy: dir.y(), x: (start.get() % 8) as i8, y: (start.get() / 8) as i8 }
    }
}

#[cfg(test)]
mod test {
    use crate::chess::{board::Board, direction::Direction};

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
