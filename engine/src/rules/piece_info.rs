use std::default::default;

use crate::chess::direction::Direction;
use crate::chess::square::Square;
use crate::chess::Team;
use crate::move_gen::moves::Moves;
use crate::state::board_state::BoardState;

pub type Piece<'a> = &'a PieceInfo;
pub trait MoveGenFn = Fn(&BoardState, &mut Moves, Square, Team) -> ();
// pub type SpecialBehaviorFn = &'static dyn Fn(&State, Move) -> ();

/// Information describing a piece
#[derive(Default, Clone)]
pub struct PieceInfo {
    /// Name of the piece
    pub name: String,
    /// Character used to represent the piece
    pub ch: &'static str,
    /// Character used to represent the piece in a FEN string
    pub fen_ch: Option<char>,
    /// Team that the piece belongs to
    pub team: Team,
    /// Value of the piece
    pub value: u8,
    // /// Moves that the piece can make
    // pub moves: Vec<(i8, i8)>,
    /// Directions that the piece can attack in
    pub attacks: Vec<Direction>,
    /// Function that generates the moves for the piece initially
    pub move_gen: Option<&'static dyn MoveGenFn>,
}

impl std::fmt::Debug for PieceInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PieceInfo")
            .field("name", &self.name)
            .field("ch", &self.ch)
            .field("team", &self.team)
            .field("value", &self.value)
            // .field("moves", &self.moves)
            .field("attacks", &self.attacks)
            .field(
                "move_gen",
                if self.move_gen.is_some() {
                    &"yes"
                } else {
                    &"no"
                },
            )
            // .field("special_behavior", if self.special_behavior.is_some() { &"yes" } else { &"no" })
            .finish()
    }
}

impl PieceInfo {
    pub fn new() -> PieceInfo {
        PieceInfo {
            name: "NOT SET".to_string(),
            ch: "?",
            ..default()
        }
    }

    proc_macros::builder_impl!(
        /// Set the name of the piece
        pub fn name<T: Into<String>>(name: T) => name.into();
        /// Set the character that represents the piece
        pub fn ch(ch: &'static str);
        /// Set the character that represents the piece in a FEN string
        pub fn fen_ch(ch: char) => Some(ch);
        /// Set the affiliation of the piece
        pub fn team(team: Team);
        /// Set the value of the piece
        pub fn value(value: u8);
        // /// Set the moves that the piece can make
        // pub fn moves(moves: &[(i8, i8)]) => moves.to_vec();
        /// Set the directions that the piece can attack in
        pub fn attacks(attacks: &[Direction]) => attacks.to_vec();
        /// Set the special cases for moves
        pub fn move_gen<T: MoveGenFn>(move_gen: &'static T) => Some(move_gen);
        // /// Set the special behavior after moving
        // pub fn special_behavior(special_behavior: SpecialBehaviorFn) => Some(special_behavior);
    );
}
