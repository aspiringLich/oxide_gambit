use super::{board::Board, moves::Moves, pieces::Piece, square::Square};
use std::fmt::Debug;
use yauc::{prelude::*, wyz::FmtForward};

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
#[derive(Default)]
pub struct State {
    /// stores the position of all the pieces
    pub board: Board,
    // stores the moves for each piece
    pub moves: Vec<Option<Moves>>,
    // stores the team that is currently moving (white or black)
    pub turn: Team,
}

impl State {
    /// get the piece at a square
    pub fn get_board<T: Into<u8>>(&self, index: T) -> &Piece {
        &self.board[Square::new(index.into())]
    }

    /// get the piece at a square but mutable
    pub fn get_board_mut<T: Into<u8>>(&mut self, index: T) -> &mut Piece {
        &mut self.board[Square::new(index.into())]
    }

    /// adds a piece to the board
    pub fn add_piece_char<T: Into<u8>>(&mut self, ch: char, pos: T, id: u8) {
        *self.get_board_mut(pos.into()) = Piece::from_char(ch, id).unwrap();
        self.moves.push(Some(Moves::default()));
    }
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out: Vec<String> = vec![style!("\nBoard: 8 ", bold, blue)];
        out.extend(vec!["       ".to_string(); 7].iter().enumerate().map(|(i, s)| {
            style!(("{}{} ", s, char::from_digit(7 - i as u32, 8).unwrap()), bold, blue)
        }));
        out.push(style!("          A  B  C  D  E  F  G  H\nMoves:", bold, blue));
        for pos in 0..64 {
            let piece = self.get_board((7 - pos / 8) * 8 + pos % 8);
            // if theres a piece on this square
            let write = if piece.is_some() {
                let ret = format!("{}{:2}", piece.to_emoji(), piece.get_id());
                let ret = match piece.team() {
                    Team::Black => style!(ret, red),
                    Team::White => style!(ret, blue),
                };
                let moves = self.moves[piece.get_id() as usize].as_ref().unwrap();
                let cls = |s, v: &Vec<Square>| {
                    if v.is_empty() {
                        format!("")
                    } else {
                        format!("{}{}", s, v.clone().fmt_list())
                    }
                };
                let mstr =
                    format!("{} {} {}", ret, cls("p:", &moves.priority), cls("n:", &moves.other));
                out.push(mstr);
                ret
            } else {
                style!("⛶  ", dark_gray)
            };
            out[pos as usize / 8] += &write;
        }

        f.write_str(&out.join("\n"))
    }
}
