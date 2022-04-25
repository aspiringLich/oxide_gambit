
/// cool structure for storing a pieces type
#[derive(Debug, Copy, Clone)]
pub struct PieceType(pub u8);

impl PieceType {
    pub const fn new(id: u8) -> Self {
        PieceType(id)
    }

    pub const fn team(&self) -> bool {
        if self.0 & (1 << 7) == 0 { return false }
        true
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


/// cool structure for storing pieces
#[derive(Debug)]
pub struct Piece {
    pos: u8,
    id: PieceType,
}

impl Piece {
    pub const fn new(pos: u8, id: PieceType) -> Self {
        Piece { 
            pos: pos,
            id: id,
        }
    }

    pub fn from_char(pos: u8, ch: char) -> Self {
        Piece { 
            pos: pos,
            id: PieceType::from_char(ch),
        }
    }

    pub const fn team(&self) -> bool {
        self.id.team()
    }
}