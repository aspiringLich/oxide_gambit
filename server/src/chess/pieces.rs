use std::ops::{Deref, DerefMut};

use super::state::Team;
use anyhow::{anyhow, Result};
use paste::paste;
use yauc::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, IntoPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum PieceType {
    #[default]
    None,
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
pub const PIECE_NUM: u8 = 6;

impl PieceType {
    /// make a piece from a character (as you would see in a FEN string)
    pub fn from_char(ch: char) -> Result<Self> {
        let mut piece: u8 = match ch.to_lowercase().to_string().as_bytes()[0] as char {
            'p' => PAWN,
            'r' => ROOK,
            'n' => KNIGHT,
            'b' => BISHOP,
            'q' => QUEEN,
            'k' => KING,
            _ => return Err(anyhow!("Invalid character")),
        };
        if ch.is_uppercase() {
            piece += 6;
        }

        Ok(Self::from(1 + piece))
    }

    /// return the affilation of the piece
    pub fn team(self) -> Team {
        Team::from((*self >= *Self::WPawn) as u8)
    }

    /// return the type of piece it is (0..=5)
    pub fn piece(self) -> u8 {
        (*self - 1) % PIECE_NUM
    }

    /// is a piece
    pub fn occupied(self) -> bool {
        self != PieceType::None
    }

    /// turn the piece into a little emoji :)
    pub fn to_emoji(self) -> char {
        match self {
            Self::None => '!',
            Self::BPawn => '♟',
            Self::BRook => '♜',
            Self::BKnight => '♞',
            Self::BBishop => '♝',
            Self::BQueen => '♛',
            Self::BKing => '♚',
            Self::WPawn => '♟',
            Self::WRook => '♜',
            Self::WKnight => '♞',
            Self::WBishop => '♝',
            Self::WQueen => '♛',
            Self::WKing => '♚',
        }
    }
}

impl Deref for PieceType {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        // is there a better way to do this? probably. but it works lol
        unsafe { std::mem::transmute(self) }
    }
}

#[test]
/// test the basic functions associated with PieceType
fn piecetype_basic() {
    // size
    assert_eq!(std::mem::size_of::<PieceType>(), 1);

    use PieceType::*;
    use Team::*;

    // team
    assert_eq!(WPawn.team(), White);
    assert_eq!(WKing.team(), White);
    assert_eq!(BPawn.team(), Black);
    assert_eq!(BKing.team(), Black);

    // piece type
    assert_eq!(WPawn.piece(), PAWN);
    assert_eq!(WKing.piece(), KING);
    assert_eq!(BPawn.piece(), PAWN);
    assert_eq!(BKing.piece(), KING);

    // from_char()
    assert_eq!(PieceType::from_char('p').unwrap(), BPawn);
    assert_eq!(PieceType::from_char('k').unwrap(), BKing);
    assert_eq!(PieceType::from_char('P').unwrap(), WPawn);
    assert_eq!(PieceType::from_char('K').unwrap(), WKing);
}

/// holds information about a piece : namely its id and its type
///
/// gotta save those bytes!!!!
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Piece {
    id: u8,
    r#type: PieceType,
}

impl Piece {
    /// create a new piece from an id (ms 4 bits) and the piece type (ls 4 bits)
    pub fn new(id: u8, r#type: PieceType) -> Self {
        Piece { id, r#type }
    }

    /// get the id of the piece and do some fun indexing stuff
    pub fn get_id(self) -> u8 {
        self.id
    }

    /// get the id of the piece mutably and do some fun indexing stuff
    pub fn get_id_mut(&mut self) -> &mut u8 {
        &mut self.id
    }

    /// create piece from a char
    pub fn from_char(ch: char, id: u8) -> Result<Self> {
        let r#type = PieceType::from_char(ch)?;
        Ok(Self::new(id, r#type))
    }
}

impl Deref for Piece {
    type Target = PieceType;

    fn deref(&self) -> &Self::Target {
        &self.r#type
    }
}

impl DerefMut for Piece {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.r#type
    }
}

/// Holds the index of every piece
#[derive(Default, Debug, Clone)]
pub struct Pieces {
    inner: [Vec<u8>; 6],
}

macro_rules! hyperspecific_get_pieces {
    ($indx:expr, $name:ident) => {
        paste! {
            pub fn [<get_ $name>](&self) -> &Vec<u8> {
                &self.inner[$indx as usize]
            }

            pub fn [<get_ $name _mut>](&mut self) -> &mut Vec<u8> {
                &mut self.inner[$indx as usize]
            }
        }
    };
}

impl Pieces {
    /// get the index of every piece of this type
    pub fn get_squares_of(&self, piece: u8) -> &Vec<u8> {
        debug_assert!((piece as usize) < self.inner.len());
        &self.inner[piece as usize]
    }

    /// get a &mut to the index of every piece of this type
    pub fn get_squares_mut_of(&mut self, piece: u8) -> &mut Vec<u8> {
        debug_assert!((piece as usize) < self.inner.len());
        &mut self.inner[piece as usize]
    }

    /// get the index of this piece
    pub fn get_square_of(&self, piece: u8, index: u8) -> u8 {
        debug_assert!((piece as usize) < self.inner.len());
        debug_assert!((index as usize) < self.inner[piece as usize].len());
        self.inner[piece as usize][index as usize]
    }

    /// get a &mut to the index of this piece
    pub fn get_square_mut_of(&mut self, piece: u8, index: u8) -> &mut u8 {
        debug_assert!((piece as usize) < self.inner.len());
        debug_assert!((index as usize) < self.inner[piece as usize].len());
        &mut self.inner[piece as usize][index as usize]
    }

    hyperspecific_get_pieces!(PAWN, pawn);
    hyperspecific_get_pieces!(ROOK, rook);
    hyperspecific_get_pieces!(KNIGHT, knight);
    hyperspecific_get_pieces!(BISHOP, bishop);
    hyperspecific_get_pieces!(QUEEN, queen);
    hyperspecific_get_pieces!(KING, king);
}
