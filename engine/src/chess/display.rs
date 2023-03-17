use std::fmt::Display;

use crossterm::style::Stylize;

use crate::chess::Team;

use super::state::State;

impl Display for State<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut white_pieces = String::new();
        let mut black_pieces = String::new();

        for idx in self.pieces.iter().skip(1) {
            let Some(piece) = self.piece_info[*idx as usize]
                .as_ref()
                else { continue };
            let push_to = match piece.team {
                Team::White => &mut white_pieces,
                Team::Black => &mut black_pieces,
            };
            *push_to += piece.ch;
        }

        let pieces = format!(
            "{} {}\n{} {}",
            "White pieces".blue(),
            white_pieces.blue(),
            "Black pieces".red(),
            black_pieces.red()
        );
        f.write_str(&pieces)
    }
}
