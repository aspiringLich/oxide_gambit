use crossterm::style::Stylize;
use std::fmt::{Display, Formatter};

use crate::{chess::Team, state::state::State};

impl Display for State<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fmt_pieces(self, f)?;
        Ok(())
    }
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
            {} {}\n{} {}",
        numbers,
        "White pieces".blue(),
        white_pieces.blue(),
        "Black pieces".red(),
        black_pieces.red()
    );
    f.write_str(&pieces)
}
