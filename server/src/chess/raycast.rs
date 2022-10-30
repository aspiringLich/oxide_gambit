use super::square::*;

use anyhow::anyhow;
use num_derive::FromPrimitive;

#[derive(Clone, Copy, Debug, FromPrimitive, PartialEq, Eq)]
pub enum Ray {
    UpL,
    Up,
    UpR,
    Left,
    None,
    Right,
    DownL,
    Down,
    DownR,
}

impl Ray {
    /// creates a new ray from an x and y value
    ///     \  |  /
    ///     -  x  -
    ///     /  |  \
    /// the function makes sure it is correct w/ `debug_assert!()`
    pub fn new(x: i8, y: i8) -> Self {
        debug_assert!((-1..=1).contains(&x));
        debug_assert!((-1..=1).contains(&y));
        debug_assert!(!(x == 0 && y == 0));

        unsafe { num::FromPrimitive::from_i8(x + 1 + (y + 1) * 3).unwrap() }
    }

    /// converts self into an xy pair
    pub const fn into_xy(self) -> (i8, i8) {
        [(-1, 1), (0, 1), (1, 1), (-1, 0), (0, 0), (1, 0), (-1, -1), (0, -1), (1, -1)]
            [self as usize]
    }
}

pub struct Raycast {
    start: Square,
    direction: Ray,
}

impl Raycast {
    pub fn new(start: Square, direction: Ray) -> Self {
        Raycast { start, direction }
    }
}

impl Iterator for Raycast {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        let updated = self.start.try_to(self.direction.into_xy());
        match updated {
            Some(pos) => {
                self.start = pos;
                Some(pos)
            }
            None => None,
        }
    }
}
