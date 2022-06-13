use super::*;
use bevy::prelude::default;

impl ChessState {
    /// loads a FEN string into the board state
    pub fn from_FEN(str: &str) -> Self {
        let mut state: ChessState = default();
        let mut section = 0; // which section of the FEN string are we on?

        // 0    => pieces on the board  "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"
        // 1    => turn                 "w"
        // 2    => castling rights      "KQkq"
        // 3    => en passant?          "-"
        // 4    => halfmove clock       "0"
        // 5    => move counter         "1"

        let mut square: u8 = 0; // square number you are on
        for ch in str.chars() {
            if ch == ' ' {
                section += 1;
                continue;
            }

            match section {
                // write down the pieces
                0 => {
                    match ch {
                        // skip <x> squares
                        '1'..='8' => square += ch as u8 - '0' as u8,
                        // next rank
                        '/' => continue,
                        // wow something else
                        _ => {
                            state.add_piece(ch, (square % 8) + (7 - (square / 8)) * 8);
                            square += 1;
                        }
                    }
                }
                // who's g dang turn is it??
                1 => {
                    if ch == 'b' {
                        state.turn = false;
                    }
                }
                2 => {}
                3 => {}
                4 => {}
                5 => {}
                _ => panic!(
                    "invalid FEN string? either double check your string is valid or i did a dumb"
                ),
            }
        }
        state.gen_threat();
        state.check_pins();
        state.move_gen();
        dbg!(&state.moves);
        return state;
    }
}
