use super::square::*;
use yauc::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive)]
#[repr(u8)]
pub enum Ray {
    #[default]
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
    ///
    /// ```compile_fail
    ///     \  |  /
    ///     -  x  -
    ///     /  |  \
    /// ```
    ///
    /// the function makes sure it is correct w/ `debug_assert!()`
    pub fn new(x: i8, y: i8) -> Self {
        debug_assert!((-1..=1).contains(&x));
        debug_assert!((-1..=1).contains(&y));
        debug_assert!(!(x == 0 && y == 0));

        Self::from((x + 1 + (y + 1) * 3) as u8)
    }

    /// converts self into an xy pair
    pub const fn into_xy(self) -> (i8, i8) {
        [(-1, 1), (0, 1), (1, 1), (-1, 0), (0, 0), (1, 0), (-1, -1), (0, -1), (1, -1)]
            [self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

#[test]
fn test_raycast() {
    let test = |x, y, direction: Ray, expect: Vec<(u8, u8)>| {
        let rc = Raycast::new(Square::from_xy((x, y)), direction);
        assert!(rc.into_iter().eq(expect.into_iter().map(|xy| Square::from_xy(xy))));
    };

    test(1, 1, Ray::UpR, vec![(2, 2), (3, 3), (4, 4), (5, 5), (6, 6), (7, 7)]);
    test(4, 4, Ray::Left, vec![(3, 4), (2, 4), (1, 4), (0, 4)]);
    test(0, 0, Ray::Down, vec![]);
}
