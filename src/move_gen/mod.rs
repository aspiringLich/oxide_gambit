use crate::{chess::{state::{State, StateIndex, Index}, square::Square, Team}, rules::piece::PieceInfo};

use self::move_gen::Moves;

pub mod attack;
pub mod move_gen;

pub fn get_piece(state: &State, pos: Square, x: i8, y: i8) -> Option<(Index<PieceInfo>, Square)> {
    let square = pos.try_move(x, y)?;
    let piece = state.board[square];
    Some((piece, square))
}

pub fn white_pawn(state: &State, moves: &mut Moves, pos: Square) {
    let (_, y) = pos.to_xy();

    // move forward
    if let Some((piece, square)) = get_piece(state, pos, 0, 1) {
        moves.insert(piece, square);

        // move forward 2 squares
        if y == 1 && let Some((piece, square)) = get_piece(state, pos, 0, 2) {
            moves.insert_good(piece, square);
        }
    }

    // capture
    if let Some((piece, square)) = get_piece(state, pos, 1, 1) {
        if state.get(piece).team != Team::White {
            moves.insert_good(piece, square);
        }
    }
    if let Some((piece, square)) = get_piece(state, pos, -1, 1) {
        if state.get(piece).team != Team::White {
            moves.insert_good(piece, square);
        }
    }
}
