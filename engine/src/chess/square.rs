use std::fmt::Display;

use crate::*;

use super::direction::Direction;

#[derive(Deref, DerefMut, Hash, PartialEq, Eq, Debug, Clone, Copy, PartialOrd, Ord)]
pub struct Square(pub u8);

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (x, y) = self.to_xy();
        write!(f, "{}{}", (x as u8 + b'a') as char, y + 1)
    }
}

impl Square {
    pub fn from_xy<T: TryInto<u8>>(x: T, y: T) -> Option<Square> {
        let (x, y) = (x.try_into().ok()?, y.try_into().ok()?);
        Self::valid_xy(x, y).then(|| Square(x + y * 8))
    }

    pub fn to_xy(&self) -> (u8, u8) {
        (self.0 % 8, self.0 / 8)
    }

    pub fn x(&self) -> u8 {
        self.0 % 8
    }

    pub fn y(&self) -> u8 {
        self.0 / 8
    }

    pub fn valid_xy(x: u8, y: u8) -> bool {
        x < 8 && y < 8
    }

    /// Returns the square thats 1 square in this direction
    pub fn dir(&self, dir: Direction) -> Option<Square> {
        let (x, y) = self.to_xy();
        let (dx, dy) = dir.xy();
        self.try_move(x as i8 + dx, y as i8 + dy)
    }

    /// Returns the square thats the result of this move
    pub fn try_move(&self, x: i8, y: i8) -> Option<Square> {
        let (_x, _y) = self.to_xy();
        let (x, y) = (_x as i8 + x, _y as i8 + y);
        Self::from_xy(x, y)
    }
}
