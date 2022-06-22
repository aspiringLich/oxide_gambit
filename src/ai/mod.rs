pub mod ai_sys;
pub use ai_sys::*;

pub mod minimax;
pub use minimax::*;

pub mod board_eval;
pub use board_eval::*;

pub mod square_table;
pub use square_table::*;

pub mod nullmove;
pub use nullmove::*;

use bevy::{prelude::*, tasks::Task};

use crate::chess_logic::move_gen::ChessMove;

#[derive(Component)]
pub struct ComputeMove(Task<ChessMove>);

const DEBUG: bool = true;

const PIECE_VALUE: [f32; 7] = [0.0, 1.0, 3.2, 3.3, 5.0, 9.0, 0.0];
const SQUARE_WEIGHT: f32 = 1.0 / 200.0;
const THREATENED_WEIGHT: f32 = 1.0 / 1.5;
const PROTECTED_WEIGHT: f32 = 1.0 / 2.0;
