use std::default::default;

use super::{ direction::Direction, board::Board, Team, move_gen::{ Moves, Move } };

pub type SpecialMoveGenFn = &'static dyn Fn(&Board<Option<Piece>>, &mut Moves) -> ();
pub type SpecialBehaviorFn = &'static dyn Fn(&mut Board<Option<Piece>>, Move) -> ();

/// Information describing a piece
#[derive(Default)]
pub struct PieceInfo {
    pub name: String,
    pub ch: char,
    pub value: u8,
    /// Moves that the piece can make
    pub moves: Vec<(i8, i8)>,
    /// Directions that the piece can attack in
    pub attacks: Vec<Direction>,
    /// Special cases for moves
    pub move_gen: Option<SpecialMoveGenFn>,
    /// Special behavior after moving
    pub special_behavior: Option<SpecialBehaviorFn>,
}

impl std::fmt::Debug for PieceInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PieceInfo")
            .field("name", &self.name)
            .field("ch", &self.ch)
            .field("value", &self.value)
            .field("moves", &self.moves)
            .field("attacks", &self.attacks)
            .field("special", if self.move_gen.is_some() { &"yes" } else { &"no" })
            .field("special_behavior", if self.special_behavior.is_some() { &"yes" } else { &"no" })
            .finish()
    }
}

impl PieceInfo {
    pub fn new() -> PieceInfo {
        PieceInfo { name: "DEFAULT".to_string(), ch: '?', ..default() }
    }
    
    /// Set the name of the piece
    pub fn name(mut self, name: &str) -> PieceInfo {
        self.name = name.to_string();
        self
    }
    
    /// Set the character used to represent the piece
    pub fn ch(mut self, ch: char) -> PieceInfo {
        self.ch = ch;
        self
    }
    
    /// Set the value of the piece
    pub fn value(mut self, value: u8) -> PieceInfo {
        self.value = value;
        self
    }
    
    /// Set the moves that the piece can make
    pub fn moves(mut self, moves: &[(i8, i8)]) -> PieceInfo {
        self.moves = moves.to_vec();
        self
    }
    
    /// Set the directions that the piece can attack in
    pub fn attacks(mut self, attacks: &[Direction]) -> PieceInfo {
        self.attacks = attacks.to_vec();
        self
    }
    
    /// Set the special move generation function
    pub fn special_move_gen(mut self, f: SpecialMoveGenFn) -> PieceInfo {
        self.move_gen = Some(f);
        self
    }
    
    /// Set the special behavior function
    pub fn special_behavior(mut self, f: SpecialBehaviorFn) -> PieceInfo {
        self.special_behavior = Some(f);
        self
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Piece {
    pub info: &'static PieceInfo,
    pub team: Team,
}