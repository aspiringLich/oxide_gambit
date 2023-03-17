use crossterm::style::{SetBackgroundColor, Stylize};
use std::fmt::{Display, Formatter};

use crate::{
    chess::{
        board::{Board, BoardType},
        index::Index,
        Team,
    },
    rules::piece::Piece,
    state::state::State,
};

fn write_style<S, T: Stylize<Styled = S> + Display>(s: T, f: &mut Formatter) -> std::fmt::Result {
    f.write_str(&format!("{}", s))
}

impl Display for State<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fmt_pieces(self, f)?;
        f.write_str("\n")?;
        write_style("Board:\n".red(), f)?;
        fmt_board(self.board_state.board(), |a, b| fmt_piece(self, a, b), f)?;
        Ok(())
    }
}

fn fmt_piece(state: &State, idx: &Index<Piece>, s: &mut String) {
    if let Some(piece) = state.board_state.get_idx(*idx) {
        let out = format!("{} ", piece.ch);
        let styled = match piece.team {
            Team::White => out.blue(),
            Team::Black => out.green(),
        };

        *s += &styled.to_string();
    } else {
        *s += "  ";
    }
}

fn fmt_board<T, F>(board: &Board<T>, dbg_fn: F, f: &mut Formatter<'_>) -> std::fmt::Result
where
    T: BoardType,
    F: Fn(&T, &mut String),
{
    let mut board_str = String::new();
    for y in (0..8).rev() {
        board_str += &format!(" {} ", y + 1);
        for x in 0..8 {
            let item = board.get(x + y * 8).unwrap();
            let mut s = String::new();
            dbg_fn(item, &mut s);
            if (x + y) % 2 == 0 {
                s = s.on_black().to_string();
            }
            board_str += &s;
        }
        board_str += "\n";
    }

    f.write_str(&format!("{}   a b c d e f g h\n", board_str))
}

fn fmt_pieces(state: &State, f: &mut Formatter<'_>) -> std::fmt::Result {
    let mut white_pieces = String::new();
    let mut black_pieces = String::new();
    let mut numbers = String::new();

    for piece in state.board_state.pieces().iter() {
        let Some(piece) = state.board_state.get_piece(*piece)
                else { continue };
        let push_to = match piece.team {
            Team::White => &mut white_pieces,
            Team::Black => &mut black_pieces,
        };
        *push_to += piece.ch;
        *push_to += " ";
    }

    let count_pieces = |s| {
        state
            .board_state
            .pieces()
            .iter()
            .filter(|&&x| {
                let Some(x) = state.board_state.get_piece(x) else { return false };
                x.team == s
            })
            .count()
    };
    let len = usize::max(count_pieces(Team::White), count_pieces(Team::Black));
    for i in 1..=len {
        numbers += &format!(" {}", char::from_digit(i as u32, 36).unwrap());
    }

    let pieces = format!(
        "            {}\n\
        {} {}\n\
        {} {}\n",
        numbers,
        "White pieces".blue(),
        white_pieces.blue(),
        "Black pieces".green(),
        black_pieces.green()
    );
    f.write_str(&pieces)
}
