pub mod board;
pub mod direction;
pub mod index;
pub mod square;

#[derive(Debug, Clone, Copy, Default, Eq)]
#[derive_const(PartialEq)]
pub enum Team {
    #[default]
    Black,
    White,
}
