use std::{
    fmt::{Display, Formatter},
    ops::{Deref, DerefMut},
};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Square {
    square: u8,
}

impl Deref for Square {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.square
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", (self.x() + 'a' as u8) as char, self.y() + 1)
    }
}

impl DerefMut for Square {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.square
    }
}

impl From<Square> for usize {
    fn from(square: Square) -> Self {
        square.square as usize
    }
}

impl Square {
    /// Creates a new square
    pub fn new(square: u8) -> Self {
        Square { square }
    }

    /// creates a new square from xy
    pub fn from_xy(xy: (u8, u8)) -> Self {
        let (x, y) = xy;
        Square::new(x + y * 8)
    }

    /// returns the x value of the square
    pub fn x(self) -> u8 {
        *self % 8
    }

    /// returns the y value of the square
    pub fn y(self) -> u8 {
        *self / 8
    }

    /// returns and x and the y value of the square as a tuple
    pub fn xy(self) -> (u8, u8) {
        (self.x(), self.y())
    }

    /// tries to move the Square to the *relative* position xy
    pub fn try_to(self, xy: (i8, i8)) -> Option<Self> {
        let (mut x, mut y) = xy;
        x += self.x() as i8;
        y += self.y() as i8;

        if x < 0 || y < 0 {
            return None;
        }
        if x > 7 || y > 7 {
            return None;
        }
        Some(Self::from_xy((x as u8, y as u8)))
    }
}

/// test the try_to() function, which uses most of the simpler functions, so is a good test of all of them
#[test]
fn square_try_to() {
    let expect_some = |x1, y1, x2, y2, ex, ey| {
        assert_eq!(Square::from_xy((x1, y1)).try_to((x2, y2)), Some(Square::from_xy((ex, ey))))
    };
    let expect_none = |x1, y1, x2, y2| assert_eq!(Square::from_xy((x1, y1)).try_to((x2, y2)), None);

    expect_some(0, 0, 1, 1, 1, 1);
    expect_some(5, 5, -1, -2, 4, 3);
    expect_some(7, 7, -7, -7, 0, 0);
    expect_some(3, 6, -1, 1, 2, 7);
    expect_none(1, 1, -2, -2);
    expect_none(3, 4, 4, 4);
    expect_none(0, 0, -1, 7);
    expect_none(4, 0, 4, -14);
}
