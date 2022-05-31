use super::{PieceType, PieceVariant, Position};

/// represents a piece, with a type and position
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Piece {
    pub variant: PieceType,
    pub position: Position,
}

impl Piece {
    pub fn new(variant: PieceType, position: Position) -> Self {
        Piece { variant, position }
    }

    pub const fn team(&self) -> bool {
        self.variant.team()
    }

    pub const fn variant(&self) -> PieceVariant {
        self.variant.variant()
    }

    // unchecked return y position
    pub const fn y(&self) -> u8 {
        self.position.y()
    }

    // unchecked return x position
    pub const fn x(&self) -> u8 {
        self.position.x()
    }

    // try to move in a way movement specifies
    pub fn try_to(&self, movement: (i8, i8)) -> Option<Position> {
        let (x, y) = movement;
        let (x, y) = (
            u8::wrapping_add(self.x(), x.to_be_bytes()[0]),
            u8::wrapping_add(self.y(), y.to_be_bytes()[0]),
        );
        let out = if x >= 8 || y >= 8 { None } else { Some(Position(x + y * 8)) };
        out
    }
}

impl Default for Piece {
    fn default() -> Self {
        Self { variant: Default::default(), position: Position(0) }
    }
}

// /// cool structure for storing a pieces type
// #[derive(Debug, Copy, Clone, PartialEq, Eq)]
// pub struct PieceType(pub u8);

// impl PieceType {
//     pub const fn new(id: u8) -> Self {
//         PieceType(id)
//     }

//     pub const fn team(&self) -> bool {
//         if self.0 & (1 << 7) == 0 {
//             return false;
//         }
//         true
//     }

//     pub const fn piece_id(&self) -> u8 {
//         self.0 & 0x7F
//     }

//     pub fn from_char(ch: char) -> Self {
//         let team = if ch as u8 > 'a' as u8 { 0x00 } else { 0x80 };
//         let piece = match ch.to_lowercase().to_string().as_bytes()[0] as char {
//             'p' => 1,
//             'r' => 2,
//             'n' => 3,
//             'b' => 4,
//             'k' => 5,
//             'q' => 6,
//             _ => 0,
//         };
//         PieceType(team | piece)
//     }
// }

// // impl Position {
// //     pub fn pos(&self, n: u8) -> u8 {
// //         use Position::*;
// //         (match self {
// //             // position n squares <direction> relative to current position
// //             Up(i) => n as i8 - 8 * i,
// //             Down(i) => n as i8 + 8 * i,
// //             Left(i) => n as i8 - i,
// //             Right(i) => n as i8 + i,
// //             DiagTL(i) => n as i8 - 9 * i,
// //             DiagTR(i) => n as i8 - 7 * i,
// //             DiagBL(i) => n as i8 + 7 * i,
// //             DiagBR(i) => n as i8 + 9 * i,
// //             // returns rank or file
// //             File => return n % 8,
// //             Rank => return n / 8,
// //         }) as u8
// //     }
// // }

// /// cool structure for storing pieces
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub struct Piece {
//     pub pos: Pos,
//     pub id: PieceType,
// }

// impl Piece {
//     pub const fn new(pos: Pos, id: PieceType) -> Self {
//         Piece { pos, id }
//     }

//     pub const fn team(&self) -> bool {
//         self.id.team()
//     }

//     pub const fn piece_id(&self) -> u8 {
//         self.id.piece_id()
//     }
// }
