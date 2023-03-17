use crate::{
    chess::{
        square::Square,
        state::{Index, State },
        Team,
    },
    rules::piece::Piece,
};

use self::move_gen::Moves;

pub mod attack;
pub mod move_gen;

#[inline(always)]
pub fn get_piece<'a>(state: &State, pos: Square, x: i8, y: i8) -> Option<(Index<Piece>, Square)> {
    let square = pos.try_move(x, y)?;
    let idx = state.board[square];
    Some((idx, square))
}

#[inline(always)]
pub fn knight(state: &State, moves: &mut Moves, pos: Square, team: Team) {
    let mut add_move = |x, y| {
        if let Some((idx, square)) = get_piece(state, pos, x, y) {
            let piece = state.get_piece(idx);
            if let Some(piece) = piece {
                if piece.team != team {
                    moves.insert(idx, square);
                }
            } else {
                moves.insert(idx, square)
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
pub fn king(state: &State, moves: &mut Moves, pos: Square, team: Team) {
    let mut add_move = |x, y| {
        if let Some((idx, square)) = get_piece(state, pos, x, y) {
            let piece = state.get_piece(idx);
            if let Some(piece) = piece {
                if piece.team != team {
                    moves.insert(idx, square);
                }
            } else {
                moves.insert(idx, square)
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
pub fn pawn(state: &State, moves: &mut Moves, pos: Square, team: Team) {
    let (_, y) = pos.to_xy();
    let dir = match team {
        Team::White => 1,
        Team::Black => -1,
    };

    // move forward
    if let Some((piece, square)) = get_piece(state, pos, 0, 1 * dir) {
        moves.insert(piece, square);

        // move forward 2 squares
        if y == [6, 1][team as usize] && let Some((piece, square)) = get_piece(state, pos, 0, 2 * dir) {
            moves.insert_good(piece, square);
        }
    }

    // capture
    if let Some((idx, square)) = get_piece(state, pos, 1, 1 * dir) {
        if let Some(piece) = state.get_piece(idx) && piece.team != team {
            moves.insert_good(idx, square);
        }
    }
    if let Some((idx, square)) = get_piece(state, pos, -1, 1 * dir) {
        if let Some(piece) = state.get_piece(idx) && piece.team != team {
            moves.insert_good(idx, square);
        }
    }
}