/// acts as a wrapper for a u8, representing a position on a chessboard
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position(pub u8);

impl Position {
    pub const fn new(n: u8) -> Self {
        Position(n)
    }

    /// return the rank of the chessboard the piece is on (0..=7)
    pub const fn y(self) -> u8 {
        self.0 / 8
    }

    /// return the file of the chessboard the piece is on (0..=7)
    pub const fn x(self) -> u8 {
        self.0 % 8
    }
}

impl Default for Position {
    fn default() -> Self {
        Position(0)
    }
}
