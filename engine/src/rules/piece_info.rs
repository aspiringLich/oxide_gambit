use std::default::default;

use crate::chess::direction::Direction;
use crate::chess::Team;

// pub type Piece<'a> = &'a PieceInfo;
// pub trait MoveGenFn = Fn(&BoardState, &mut Moves, Square, Team) -> ();
// pub type SpecialBehaviorFn = &'static dyn Fn(&State, Move) -> ();

/// Information describing a piece
#[derive(Default, Clone, Debug)]
pub struct PieceInfo {
    /// Name of the piece
    pub name: &'static str,
    /// Character used to represent the piece
    pub ch: &'static str,
    /// Character used to represent the piece in a FEN string
    pub fen_ch: Option<char>,
    /// Value of the piece
    pub value: u8,
    /// Directions that the piece can attack in
    pub attacks: Vec<Direction>,
    /// Team that the piece belongs to
    pub team: Team,
    /// Index into the sprite sheet
    pub sprite_index: usize,
}

impl PieceInfo {
    pub fn new() -> PieceInfo {
        PieceInfo {
            name: "NOT SET",
            ch: "?",
            ..default()
        }
    }

    proc_macros::builder_impl!(
        /// Set the name of the piece
        pub fn name(name: &'static str) => name;
        /// Set the character that represents the piece
        pub fn ch(ch: &'static str);
        /// Set the character that represents the piece in a FEN string
        pub fn fen_ch(ch: char) => Some(ch);
        /// Set the value of the piece
        pub fn value(value: u8);
        /// Set the directions that the piece can attack in
        pub fn attacks(attacks: &[Direction]) => attacks.to_vec();
        /// Set the index of the piece in the sprite sheet
        pub fn sprite_index(index: usize);
    );

    pub fn build(mut self, team: Team) -> Self {
        self.team = team;
        match team {
            Team::White => {}
            Team::Black => {
                self.attacks = self.attacks.into_iter().map(|d| d.flip_y()).collect();
                self.fen_ch = self.fen_ch.map(|c| {
                    c.to_uppercase()
                        .next()
                        .expect("set FEN character has uppercase variant(s)")
                });
            }
        }
        self.attacks.sort_by_key(|d| *d as u8);
        self
    }
}
