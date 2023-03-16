use crate::{
    chess::{
        square::Square,
        state::{Index, State, StateIndex},
        Team,
    },
    rules::piece::PieceInfo,
};

use self::move_gen::Moves;

pub mod attack;
pub mod move_gen;

pub fn get_piece(state: &State, pos: Square, x: i8, y: i8) -> Option<(Index<PieceInfo>, Square)> {
    let square = pos.try_move(x, y)?;
    let piece = state.board[square];
    Some((piece, square))
}

pub fn white_pawn(state: &State, moves: &mut Moves, pos: Square) {
    pawn::<1, { Team::White }>(state, moves, pos);
}

pub fn black_pawn(state: &State, moves: &mut Moves, pos: Square) {
    pawn::<-1, { Team::Black }>(state, moves, pos);
}

pub fn knight(state: &State, moves: &mut Moves, pos: Square) {
    let mut add_move = |x, y| {
        if let Some((piece, square)) = get_piece(state, pos, x, y) {
            moves.insert(piece, square);
        }
    };

    add_move(1, 2);
    add_move(2, 1);
    add_move(2, -1);
    add_move(1, -2);
    add_move(-1, -2);
    add_move(-2, -1);
    add_move(-2, 1);
    add_move(-1, 2);
}

pub fn king(state: &State, moves: &mut Moves, pos: Square) {
    let mut add_move = |x, y| {
        if let Some((piece, square)) = get_piece(state, pos, x, y) {
            moves.insert(piece, square);
        }
    };

    add_move(1, 0);
    add_move(1, 1);
    add_move(0, 1);
    add_move(-1, 1);
    add_move(-1, 0);
    add_move(-1, -1);
    add_move(0, -1);
    add_move(1, -1);
}

fn pawn<const DIR: i8, const TEAM: Team>(state: &State, moves: &mut Moves, pos: Square) {
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
