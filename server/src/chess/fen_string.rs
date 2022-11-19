use super::{square::*, state::*};
use yauc::prelude::*;

pub fn std_position_to_pos(rank: char, file: char) -> Option<Square> {
    assert!(file.is_ascii_digit());
    assert!(rank.is_ascii_alphabetic() && rank.is_ascii_lowercase());

    Some(Square::new(rank as u8 - 'a' as u8 + (file as u8 - '0' as u8) * 8))
}

impl State {
    /// loads a FEN string into the board state
    ///
    /// |#|description|example|
    /// |---|---|---|
    /// |0| pieces on the board|rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR|
    /// |1| turn|w|
    /// |2| castling rights|KQkq|
    /// |3| en passant|-|
    /// |4| halfmove clock|0|
    /// |5| move counter|1|
    ///
    /// # Example
    ///
    /// ```no_compile
    /// // returns the standard chess starting position
    /// from_FEN("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    /// ```
    ///
    /// TODO: implement other things
    #[allow(non_snake_case)]
    pub fn from_FEN(str: &str) -> Result<Self> {
        let mut state: State = default();
        let mut sections = str.split(" ").filter(|s| !s.is_empty());

        let piece_section = sections.next().expect("piece section exists");
        let mut square = 0;
        let mut id = 0;
        for ch in piece_section.chars() {
            match ch {
                // skip <x> squares
                '1'..='8' => square += ch as u8 - '0' as u8,
                // next rank
                '/' => {}
                // wow something else
                _ => {
                    // dbg!(ch, square);
                    state.add_piece_char(ch, (square % 8) + (7 - (square / 8)) * 8, id);
                    id += 1;
                    square += 1;
                }
            }
        }

        // who's g dang turn is it??
        let turn_section = sections.next().expect("turn section exists");
        let ch = turn_section.chars().next().expect("turn section has a char");
        state.turn = match ch {
            'b' => Team::Black,
            'w' => Team::White,
            _ => bail!("invalid turn section"),
        };

        // castling rights
        let castling_section = sections.next().expect("castling section exists");
        for ch in castling_section.chars() {
            match ch {
                // TODO: implement castling
                // 'q' => state.castling[0] = true,
                // 'k' => state.castling[1] = true,
                // 'Q' => state.castling[2] = true,
                // 'K' => state.castling[3] = true,
                'q' | 'Q' | 'k' | 'K' => {}
                '-' => {}
                _ => bail!("invalid castling section"),
            };
        }

        // TODO: implement en passant
        // en passant
        let en_passant_section = sections.next().expect("en passant section exists");
        for [rank, file] in en_passant_section.chars().array_chunks() {
            // state.en_passant = Some(std_position_to_pos(rank, file)?)
        }

        // TODO: implement halfmove clock
        // halfmove clock
        let halfmove_section = sections.next().expect("halfmove section exists");
        // state.halfmove_clock = halfmove_section.parse::<u32>()?;

        // TODO: implement fullmove counter
        // fullmove counter
        let fullmove_section = sections.next().expect("fullmove section exists");
        // state.fullmove_counter = fullmove_section.parse::<u32>()?;

        if let Some(err) = sections.next() {
            bail!("encountered too many sections in FEN string")
        }

        state.regenerate_moves();
        dbg!(&state);
        Ok(state)
    }
}
