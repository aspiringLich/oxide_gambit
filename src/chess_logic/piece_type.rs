#[derive(PartialEq, Copy, Clone, Debug)]
pub enum PieceVariant {
    None = 0,
    Pawn,
    Rook,
    Knight,
    Bishop,
    King,
    Queen,
}

impl Default for PieceVariant {
    fn default() -> Self {
        Self::None
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct PieceType(pub bool, pub PieceVariant);

impl PieceType {
    pub const fn team(&self) -> bool {
        self.0
    }

    pub const fn variant(&self) -> PieceVariant {
        self.1
    }

    pub fn from_char(ch: char) -> Self {
        use super::PieceVariant::*;
        let variant: PieceVariant = match ch.to_lowercase().to_string().chars().next() {
            Some('p') => Pawn,
            Some('r') => Rook,
            Some('n') => Knight,
            Some('b') => Bishop,
            Some('k') => King,
            Some('q') => Queen,
            _ => None,
        };
        let n = ch as u32;
        PieceType((n >= 'A' as u32) && (n <= 'Z' as u32), variant)
    }
}

impl Default for PieceType {
    fn default() -> Self {
        Self(false, PieceVariant::None)
    }
}
