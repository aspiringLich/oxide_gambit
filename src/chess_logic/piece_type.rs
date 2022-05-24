#[derive(PartialEq, Copy, Clone, Debug)]
pub enum PieceVariant {
    None,
    Pawn,
    Rook,
    Knight,
    Bishop,
    King,
    Queen,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct PieceType(bool, PieceVariant);

impl PieceType {
    pub fn team(&self) -> bool {
        self.0
    }

    pub fn variant(&self) -> PieceVariant {
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
