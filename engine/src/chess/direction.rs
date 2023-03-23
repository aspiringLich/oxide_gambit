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
    SE,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Direction::E => "E",
            Direction::NE => "NE",
            Direction::N => "N",
            Direction::NW => "NW",
            Direction::W => "W",
            Direction::SW => "SW",
            Direction::S => "S",
            Direction::SE => "SE",
        })
    }
}

use Direction::*;

use super::board::{self, Board};
impl Direction {
    pub const ORTHOGONAL: [Direction; 4] = [E, N, W, S];
    pub const DIAGONAL: [Direction; 4] = [NE, NW, SW, SE];
    pub const ALL: [Direction; 8] = [E, NE, N, NW, W, SW, S, SE];

    pub fn xy(self) -> (i8, i8) {
        match self {
            E => (1, 0),
            NE => (1, 1),
            N => (0, 1),
            NW => (-1, 1),
            W => (-1, 0),
            SW => (-1, -1),
            S => (0, -1),
            SE => (1, -1),
        }
    }

    pub fn x(self) -> i8 {
        self.xy().0
    }

    pub fn y(self) -> i8 {
        self.xy().1
    }

    pub fn flip_y(self) -> Self {
        match self {
            E => E,
            NE => SE,
            N => S,
            NW => SW,
            W => W,
            SW => NW,
            S => N,
            SE => NE,
        }
    }
}

/// An iterator over a given direction in a board
pub struct DirectionIter<'a, T: board::BoardType> {
    board: &'a Board<T>,
    dx: i8,
    dy: i8,
    x: i8,
    y: i8,
}

impl<'a, T: board::BoardType> Iterator for DirectionIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.x < 0 || self.x >= 8 || self.y < 0 || self.y >= 8 {
            return None;
        }
        let out = Some(self.board.get((self.y * 8 + self.x) as usize).unwrap());
        self.x += self.dx;
        self.y += self.dy;
        out
    }
}

/// A mutable iterator over a given direction in a board
pub struct DirectionIterMut<'a, T: board::BoardType> {
    board: &'a mut Board<T>,
    dx: i8,
    dy: i8,
    x: i8,
    y: i8,
}

impl<'a, T: board::BoardType> Iterator for DirectionIterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.x < 0 || self.x >= 8 || self.y < 0 || self.y >= 8 {
            return None;
        }
        let out = self.board.get_mut((self.y * 8 + self.x) as usize).unwrap();
        self.x += self.dx;
        self.y += self.dy;
        // https://stackoverflow.com/questions/63437935/in-rust-how-do-i-create-a-mutable-iterator
        // should be fine
        unsafe { Some(&mut *(out as *mut T)) }
    }
}

impl<T: board::BoardType> Board<T> {
    pub fn iter_direction<'a, I: board::BoardIndex>(
        &'a self,
        dir: Direction,
        start: I,
    ) -> DirectionIter<'a, T> {
        DirectionIter {
            board: self,
            dx: dir.x(),
            dy: dir.y(),
            x: (start.get() % 8) as i8,
            y: (start.get() / 8) as i8,
        }
    }

    pub fn iter_direction_mut<'a, I: board::BoardIndex>(
        &'a mut self,
        dir: Direction,
        start: I,
    ) -> DirectionIterMut<'a, T> {
        DirectionIterMut {
            board: self,
            dx: dir.x(),
            dy: dir.y(),
            x: (start.get() % 8) as i8,
            y: (start.get() / 8) as i8,
        }
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

        assert_eq!(
            board
                .iter_direction(Direction::NE, 0)
                .copied()
                .collect::<Vec<_>>(),
            [0, 9, 18, 27, 36, 45, 54, 63]
        );
        assert_eq!(
            board
                .iter_direction_mut(Direction::NE, 0)
                .map(|x| *x)
                .collect::<Vec<_>>(),
            [0, 9, 18, 27, 36, 45, 54, 63]
        );
        assert_eq!(
            board
                .iter_direction(Direction::NW, 3)
                .copied()
                .collect::<Vec<_>>(),
            [0o03, 0o12, 0o21, 0o30]
        );
        assert_eq!(
            board
                .iter_direction(Direction::NE, 3)
                .copied()
                .collect::<Vec<_>>(),
            [0o03, 0o14, 0o25, 0o36, 0o47]
        );
    }
}
