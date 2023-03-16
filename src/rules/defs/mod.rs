pub mod standard;

use crate::{
    chess::{square::Square, Team},
    rules::{PieceInfo, Rules},
};

pub fn white_pawn() -> PieceInfo {
    PieceInfo::new()
        .ch("♟︎")
        .value(1)
        .team(Team::White)
        .move_gen(&|state, moves, pos| {
            let (x, y) = pos.to_xy();

            // move forward
            if let Some(square) = Square::from_xy(x, y + 1) {}
        })
}
