use std::ops::Deref;

use num;
use num_derive::FromPrimitive;

use anyhow::{anyhow, Result};

#[derive(Clone, Copy, Debug, FromPrimitive, PartialEq, Eq)]
pub enum Piece {
    BPawn,
    BRook,
    BKnight,
    BBishop,
    BQueen,
    BKing,
    WPawn,
    WRook,
    WKnight,
    WBishop,
    WQueen,
    WKing,
}

pub const PAWN: u8 = 0;
pub const ROOK: u8 = 1;
pub const KNIGHT: u8 = 2;
pub const BISHOP: u8 = 3;
pub const QUEEN: u8 = 4;
pub const KING: u8 = 5;

impl Piece {
    /// make a piece from a character (as you would see in a FEN string)
    pub fn from_char(ch: char) -> Result<Self> {
        let team: u8 = if ch as u8 >= 'a' as u8 { 0 } else { 6 };
        let piece: u8 = match ch.to_lowercase().to_string().as_bytes()[0] as char {
            'p' => PAWN,
            'r' => ROOK,
            'n' => KNIGHT,
            'b' => BISHOP,
            'q' => QUEEN,
            'k' => KING,
            _ => return Err(anyhow!("Invalid character")),
        };

        unsafe {
            num::FromPrimitive::from_u8(team + piece)
                .ok_or(anyhow!("Failed to convert piece from integer"))
        }
    }

    /// return the affilation of the piece
    pub fn team(&self) -> bool {
        **self & 0x08 == 0x08
    }

    /// return the type of piece it is (0..=5)
    pub fn piece(&self) -> u8 {
        **self & 0x07
    }
}

impl Deref for Piece {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        // is there a better way to do this? probably. but it works lol
        unsafe { std::mem::transmute(self) }
    }
}

#[test]
/// test the basic functions associated with piece
fn piece_basic() {
    // size
    assert_eq!(std::mem::size_of::<Piece>(), 1);
    assert_eq!(std::mem::size_of::<Option<Piece>>(), 1);

    use Piece::*;

    // team
    assert_eq!(WPawn.team(), true);
    assert_eq!(WKing.team(), true);
    assert_eq!(BPawn.team(), false);
    assert_eq!(BKing.team(), false);

    // piece type
    assert_eq!(WPawn.piece(), PAWN);
    assert_eq!(WKing.piece(), KING);
    assert_eq!(BPawn.piece(), PAWN);
    assert_eq!(BKing.piece(), KING);

    // from_char()
    assert_eq!(Piece::from_char('p').unwrap(), BPawn);
    assert_eq!(Piece::from_char('k').unwrap(), BKing);
    assert_eq!(Piece::from_char('P').unwrap(), WPawn);
    assert_eq!(Piece::from_char('K').unwrap(), WKing);
}
