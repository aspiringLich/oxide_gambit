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

    pub const fn int(self) -> usize {
        self.0 as usize
    }

    // try to move in a way movement specifies
    pub const fn try_to(&self, movement: (i8, i8)) -> Option<Position> {
        let (x, y) = movement;
        let (x, y) = (
            u8::wrapping_add(self.x(), x.to_be_bytes()[0]),
            u8::wrapping_add(self.y(), y.to_be_bytes()[0]),
        );
        let out = if x >= 8 || y >= 8 { None } else { Some(Position(x + y * 8)) };
        out
    }

    /// position relative from a new position
    pub const fn rel_from(&self, pos: Position) -> (i8, i8) {
        ((pos.x() as i8 - self.x() as i8), (pos.y() as i8 - self.y() as i8))
    }

    pub fn modify(&mut self, input: i8) {
        *self = Position((self.0 as i8 + input) as u8);
    }
}

impl Default for Position {
    fn default() -> Self {
        Position(0)
    }
}
