use crate::State;

/// struct for holding a chess move
///     start - starting position
///     end - ending position
pub struct chess_move {
    pub start: u8, 
    pub end: u8,
}

impl State {
    pub fn move_gen(&self, team: bool) -> Vec<chess_move> {
        let moves: Vec<chess_move> = Vec![];

        unimplemented!()
    }
}