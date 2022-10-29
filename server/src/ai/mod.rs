mod ai_sys;
mod board_eval;
mod hasher;
mod minimax;
mod square_table;
mod transposition;

use bevy::{prelude::*, tasks::Task};

use crate::chess_logic::move_gen::ChessMove;

#[derive(Component)]
pub struct ComputeMove(Task<ChessMove>);

const DEBUG: bool = true;

const PIECE_VALUE: [f32; 7] = [0.0, 1.0, 3.2, 3.3, 5.0, 9.0, 0.0];
const SQUARE_WEIGHT: f32 = 1.0 / 300.0;
const THREATENED_WEIGHT: f32 = 1.0 / 6.5;
const PROTECTED_WEIGHT: f32 = 1.0 / 8.0;
