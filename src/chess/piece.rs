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

    proc_macros::builder_impl!(
        /// Set the name of the piece
        pub fn name(name: String);
        /// Set the character that represents the piece
        pub fn ch(ch: char);
        /// Set the value of the piece
        pub fn value(value: u8);
        /// Set the moves that the piece can make
        pub fn moves(moves: &[(i8, i8)]) => moves.to_vec();
        /// Set the directions that the piece can attack in
        pub fn attacks(attacks: &[Direction]) => attacks.to_vec();
        /// Set the special cases for moves
        pub fn move_gen(move_gen: SpecialMoveGenFn) => Some(move_gen);
        /// Set the special behavior after moving
        pub fn special_behavior(special_behavior: SpecialBehaviorFn) => Some(special_behavior);
    );
}

#[derive(Clone, Copy, Debug)]
pub struct Piece {
    pub info: &'static PieceInfo,
    pub team: Team,
}