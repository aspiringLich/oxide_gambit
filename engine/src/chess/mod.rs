pub mod board;
pub mod direction;
pub mod index;
pub mod square;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Team {
    #[default]
    Black,
    White,
}
