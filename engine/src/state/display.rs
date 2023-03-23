use crossterm::style::{Color, Stylize};
use std::{
    default::default,
    fmt::{Display, Formatter},
};

use crate::{
    chess::{
        board::{Board, BoardType},
        index::Index,
        square::Square,
        Team,
    },
    misc,
    rules::piece::Piece,
    state::state::State,
};

fn allocate_space() {
    print!("{}\x1b[9A", "\n".repeat(9));
}

fn reset(f: &mut Formatter) -> std::fmt::Result {
    f.write_str("\x1b[9A\x1b[1C")
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fmt_pieces(self, f)?;

        allocate_space();
        fmt_board("Board", self, self.board_state.board(), fmt_piece, f)?;
        reset(f)?;
        fmt_board("Indices", self, self.board_state.board(), fmt_index, f)?;

        println!();
        allocate_space();
        fmt_board(
            "Threats (Black)",
            self,
            self.board_state.board(),
            |a, b, c, d| fmt_threat(a, b, c, d, Team::Black),
            f,
        )?;
        reset(f)?;
        fmt_board(
            "Sliding (Black)",
            self,
            self.board_state.board(),
            |a, b, c, d| fmt_sliding(a, b, c, d, Team::Black),
            f,
        )?;
        reset(f)?;
        fmt_board(
            "Threats (White)",
            self,
            self.board_state.board(),
            |a, b, c, d| fmt_threat(a, b, c, d, Team::White),
            f,
        )?;
        reset(f)?;
        fmt_board(
            "Sliding (White)",
            self,
            self.board_state.board(),
            |a, b, c, d| fmt_sliding(a, b, c, d, Team::White),
            f,
        )?;

        write!(f, "\n\n{}", "Moves (List)\n".red())?;
        self.moves.fmt(&self.board_state, f)?;

        Ok(())
    }
}

fn fmt_piece(state: &State, idx: &Index<Piece>, _: Square, s: &mut String) {
    if let Some(piece) = state.board_state.get_info(*idx) {
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

fn fmt_sliding(state: &State, _: &Index<Piece>, square: Square, s: &mut String, team: Team) {
    let threat = *state.moves.sliding_threat_at(square, team);
    let str = if threat == 0 {
        "  ".to_string()
    } else {
        format!("{threat:2x}")
    };
    *s += &str
        .on(Color::Rgb {
            r: (255 / 8_u8)
                .checked_mul(threat.count_ones() as u8)
                .unwrap_or(255),
            g: 0,
            b: 0,
        })
        .red()
        .to_string();
}

fn fmt_threat(state: &State, _: &Index<Piece>, square: Square, s: &mut String, team: Team) {
    let threat = state.moves.threat_at(square, team);
    let str = if threat == 0 {
        "  ".to_string()
    } else {
        format!("{threat:2}")
    };
    *s += &str
        .on(Color::Rgb {
            r: (255 / 8_u8).checked_mul(threat).unwrap_or(255),
            g: 0,
            b: 0,
        })
        .red()
        .to_string();
}

fn fmt_index(_: &State, idx: &Index<Piece>, _: Square, s: &mut String) {
    let str = idx.to_string();
    if str == "0" {
        *s += "  ";
    } else {
        *s += &(str.cyan().to_string() + " ");
    }
}

fn fmt_board<T, F>(
    title: &str,
    state: &State,
    board: &Board<T>,
    dbg_fn: F,
    f: &mut Formatter<'_>,
) -> std::fmt::Result
where
    T: BoardType,
    F: Fn(&State, &T, Square, &mut String),
{
    let mut board_str = String::new();
    board_str += &format!("{}\x1b[{}D\x1b[1B", title.red(), title.len());
    for y in (0..8).rev() {
        board_str += &format!("{} ", y + 1);
        for x in 0..8 {
            let item = board.get(x + y * 8).unwrap();
            let mut s = String::new();
            dbg_fn(state, item, Square((y * 8 + x) as u8), &mut s);
            if (x + y) % 2 == 0 {
                s = s.on_black().to_string();
            }
            board_str += &format!("{}", s);
        }
        // move down and left
        board_str += "\x1b[1B\x1b[18D";
    }

    f.write_str(&format!("{}  a b c d e f g h ", board_str))
}

fn fmt_pieces(state: &State, f: &mut Formatter<'_>) -> std::fmt::Result {
    let mut strings: [(String, String); 2] = default();

    for (i, piece) in state.board_state.pieces().iter().enumerate() {
        let Some(piece) = state.board_state.get_info(*piece)
                else { continue };
        let (ref mut push_to, ref mut numbers) = &mut strings[piece.team as usize];
        *push_to += &match piece.team {
            Team::White => piece.ch.blue(),
            Team::Black => piece.ch.green(),
        }
        .to_string();
        *push_to += " ";
        *numbers += &format!("{} ", misc::u8_to_char(i as u8));
    }

    let [(black, black_nums), (white, white_nums)] = strings;
    let pieces = format!(
        "\
{}
 {black_nums}
 {black}
 {white_nums}
 {white}
",
        "Pieces".red()
    );
    f.write_str(&pieces)
}
