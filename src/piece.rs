/// cool structure for storing a pieces type
#[derive(Debug, Copy, Clone)]
pub struct PieceType(pub u8);

impl PieceType {
    pub const fn new(id: u8) -> Self {
        PieceType(id)
    }

    pub const fn team(&self) -> bool {
        if self.0 & (1 << 7) == 0 {
            return false;
        }
        true
    }

    pub const fn piece_id(&self) -> u8 {
        self.0 & 0x7F
    }

    pub fn from_char(ch: char) -> Self {
        let team = if ch as u8 > 'a' as u8 { 0x00 } else { 0x80 };
        let piece = match ch.to_lowercase().to_string().as_bytes()[0] as char {
            'p' => 1,
            'r' => 2,
            'n' => 3,
            'b' => 4,
            'k' => 5,
            'q' => 6,
            _ => 0,
        };
        PieceType(team | piece)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Position {
    Up(i8),
    Down(i8),
    Left(i8),
    Right(i8),
    DiagTL(i8),
    DiagTR(i8),
    DiagBL(i8),
    DiagBR(i8),
    Rank,
    File,
}

pub fn hacky_workaround_there_is_a_better_way_of_doing_this(n: u8, val: i8) -> Position {
    use Position::*;
    match n {
        0 => Up(val),
        1 => Down(val),
        2 => Left(val),
        3 => Right(val),
        _ => panic!("Fix your godamn code already god "),
    }
}

/// cool structure for storing pieces
#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub pos: u8,
    pub id: PieceType,
}

impl Piece {
    pub const fn new(pos: u8, id: PieceType) -> Self {
        Piece { pos: pos, id: id }
    }

    pub const fn team(&self) -> bool {
        self.id.team()
    }

    pub const fn piece_id(&self) -> u8 {
        self.id.piece_id()
    }

    /// functions for positioning related stuffs, returns a position or value
    pub const fn pos(&self, position: Position) -> u8 {
        use Position::*;
        (match position {
            // position n squares <direction> relative to current position
            Up(n) => self.pos as i8 - 8 * n,
            Down(n) => self.pos as i8 + 8 * n,
            Left(n) => self.pos as i8 - n,
            Right(n) => self.pos as i8 + n,
            DiagTL(n) => self.pos as i8 - 9 * n,
            DiagTR(n) => self.pos as i8 - 7 * n,
            DiagBL(n) => self.pos as i8 + 7 * n,
            DiagBR(n) => self.pos as i8 + 9 * n,
            // returns rank or file
            File => return self.pos % 8,
            Rank => return self.pos / 8,
        }) as u8
    }
}
