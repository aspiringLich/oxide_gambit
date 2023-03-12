

mod piece;
mod board;
mod square;
mod move_gen;
mod direction;
mod attack;
mod state;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Team {
    White,
    Black,
}