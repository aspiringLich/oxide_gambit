
pub mod board;
pub mod square;
pub mod direction;
pub mod index;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Team {
    #[default]
    Black,
    White,
}