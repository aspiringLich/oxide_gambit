/// acts as a wrapper for a u8, representing a position on a chessboard
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pos(pub u8);

impl Pos {
    /// return the rank of the chessboard the piece is on (0..=7)
    pub const fn y(self) -> u8 {
        self.0 / 8
    }

    /// return the file of the chessboard the piece is on (0..=7)
    pub const fn x(self) -> u8 {
        self.0 % 8
    }

    /// is this pos currently on the edge? if were checking in this direction
    pub const fn check_edge(self, direction: Direction) -> bool {
        use Direction::*;

        match direction {
            U => self.y() == 7,
            D => self.y() == 0,
            L => self.x() == 0,
            R => self.x() == 7,
            TL => self.check_edge(U) || self.check_edge(L),
            TR => self.check_edge(U) || self.check_edge(R),
            BL => self.check_edge(D) || self.check_edge(L),
            BR => self.check_edge(D) || self.check_edge(R),
        }
    }

    /// modify our position and return it as a u8
    pub const fn to_u8(self, x: i8, y: i8) -> u8 {
        self.0 + (x + y * 8) as u8
    }

    /// modify our pos and return it
    pub const fn to(self, direction: Direction, n: i8) -> Pos {
        use Direction::*;

        Pos({
            self.0 as i8 + {
                match direction {
                    U => 8 * n,
                    D => 8 * -n,
                    L => -n,
                    R => n,
                    TL => 7 * n,
                    TR => 9 * n,
                    BL => 9 * -n,
                    BR => 7 * -n,
                }
            }
        } as u8)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    U,
    D,
    L,
    R,
    TL,
    TR,
    BL,
    BR,
}
