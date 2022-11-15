use crate::misc::format_base64;

use super::{moves::Moves, pieces::Piece};
use std::fmt::{Debug, Display};
use yauc::{colorful::core::StrMarker, prelude::*, wyz::FmtForward};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Derivative, FromPrimitive)]
#[repr(u8)]
#[derivative(Default)]
pub enum Team {
    #[derivative(Default)]
    #[default]
    Black,
    White,
}

/// Stores the state of the chess board at any one time
#[derive(Derivative)]
#[derivative(Default)]
pub struct State {
    /// stores the position of all the pieces
    #[derivative(Default(value = "[None; 64]"))]
    board: [Option<Piece>; 64],
    // stores the moves for each piece
    pub moves: Vec<Option<Moves>>,
    // stores the team that is currently moving (white or black)
    pub turn: Team,
}

impl State {
    /// get the piece at a square
    pub fn get_board<T>(&self, index: T) -> &Option<Piece>
    where
        usize: From<T>,
    {
        &self.board[usize::from(index)]
    }

    /// get the piece at a square but mutable
    pub fn get_board_mut<T>(&mut self, index: T) -> &mut Option<Piece>
    where
        usize: From<T>,
    {
        &mut self.board[usize::from(index)]
    }

    /// adds a piece to the board
    pub fn add_piece_char<T>(&mut self, ch: char, pos: T, id: u8)
    where
        usize: From<T>,
    {
        *self.get_board_mut(pos) = Piece::from_char(ch, id).ok();
        self.moves.push(Some(Moves::default()));
    }
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let square_col = Color256::LightGray;
        let mut out: Vec<String> = vec![format!("{}", "\nBoard: 0 ".bold().blue())];
        out.extend(vec!["       ".to_string(); 7].iter().enumerate().map(|(i, s)| {
            format!("{}{} ", s, ((i as u8 + '1' as u8) as char).to_string().bold().blue())
        }));
        out.push(format!("{}", "          A  B  C  D  E  F  G  H\nMoves:".bold().blue()));
        for pos in 0..64 {
            let piece = self.get_board((7 - pos / 8) * 8 + pos % 8);
            // if theres a piece on this square
            let write = if let Some(piece) = piece {
                let ret = format!("{}{:2}", piece.to_emoji(), piece.get_id());
                let ret = match piece.team() {
                    Team::Black => ret.red(),
                    Team::White => ret.blue(),
                };
                let moves = self.moves[piece.get_id() as usize].as_ref().unwrap();
                out.push(format!(
                    "{}: !{} {}",
                    ret,
                    moves.priority.clone().fmt_list(),
                    moves.other.clone().fmt_list()
                ));
                ret
            } else {
                " ⛶ ".dark_gray()
            };
            out[pos / 8] += &format!("{}", write);
        }

        f.write_str(&out.join("\n"))
    }
}
