pub mod ai_sys;
pub use ai_sys::*;

pub mod minimax;
pub use minimax::*;

pub mod board_eval;
pub use board_eval::*;

use bevy::{prelude::*, tasks::Task};

use crate::chess_logic::move_gen::ChessMove;

#[derive(Component)]
pub struct ComputeMove(Task<ChessMove>);

const DEBUG: bool = true;
