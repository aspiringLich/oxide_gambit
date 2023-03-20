use crate::{
    chess::{index::Index, square::Square, Team},
    rules::piece::Piece,
    state::board_state::BoardState,
};

use self::moves::Moves;

pub mod attack;
pub mod generator;
pub mod moves;

#[inline(always)]
pub fn try_get_square<'a>(
    state: &BoardState,
    pos: Square,
    x: i8,
    y: i8,
) -> Option<(Index<Piece>, Square)> {
    let square = pos.try_move(x, y)?;
    let idx = state.board()[square];
    Some((idx, square))
}

#[inline(always)]
pub fn knight(state: &BoardState, moves: &mut Moves, pos: Square, team: Team) {
    let piece = state.board()[pos];
    let mut add_move = |x, y| {
        if let Some((idx, square)) = try_get_square(state, pos, x, y) {
            let p = state.get_info(idx);
            if let Some(p) = p {
                if p.team != team {
                    moves.insert(piece, square);
                }
            } else {
                moves.insert(piece, square)
            }
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

#[inline(always)]
pub fn king(state: &BoardState, moves: &mut Moves, pos: Square, team: Team) {
    let piece = state.board()[pos];
    let mut add_move = |x, y| {
        if let Some((idx, square)) = try_get_square(state, pos, x, y) {
            let p = state.get_info(idx);
            if let Some(p) = p {
                if p.team != team {
                    moves.insert(piece, square);
                }
            } else {
                moves.insert(piece, square)
            }
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

#[inline(always)]
pub fn pawn(state: &BoardState, moves: &mut Moves, pos: Square, team: Team) {
    let (_, y) = pos.to_xy();
    let dir = match team {
        Team::White => 1,
        Team::Black => -1,
    };
    let _piece = state.board()[pos];

    // move forward
    if let Some((piece, square)) = try_get_square(state, pos, 0, 1 * dir) {
        moves.insert(piece, square);

        // move forward 2 squares
        if y == [6, 1][team as usize] && let Some((_, square)) = try_get_square(state, pos, 0, 2 * dir) {
            moves.insert_good(piece, square);
        }
    }

    // capture
    if let Some((idx, square)) = try_get_square(state, pos, 1, 1 * dir) {
        if let Some(piece) = state.get_info(idx) && piece.team != team {
            moves.insert_good(idx, square);
        }
    }
    if let Some((idx, square)) = try_get_square(state, pos, -1, 1 * dir) {
        if let Some(piece) = state.get_info(idx) && piece.team != team {
            moves.insert_good(idx, square);
        }
    }
}
