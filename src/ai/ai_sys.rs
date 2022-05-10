use crate::chess_logic::{ChessMove, ChessState};
use bevy::prelude::ResMut;

pub fn ai_sys() {}

pub enum MoveAttribute {
    None,
    Checkmate,
    Stalemate,
}

pub struct MoveEvent(ChessMove, MoveAttribute);

pub fn start_calc(mut state: ResMut<ChessState>) {
    // generate moves
    //state.move_gen();
    //dbg!(&state.moves);
}
