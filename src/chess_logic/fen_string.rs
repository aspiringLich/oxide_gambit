use super::*;
use bevy::prelude::default;

pub fn std_position_to_pos(file: char, rank: char) -> Position {
    Position(rank as u8 - 'a' as u8 + (file as u8 - '0' as u8) * 8)
}

impl ChessState {
    fn FEN_placement(&mut self, ch: char, square: &mut u8) {
        match ch {
            // skip <x> squares
            '1'..='8' => *square += ch as u8 - '0' as u8,
            // next rank
            '/' => return,
            // wow something else
            _ => {
                self.add_piece(ch, (*square % 8) + (7 - (*square / 8)) * 8);
                *square += 1;
            }
        }
    }

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
        let mut prev_char: char = '`';
        let mut acc_str: String = default();
        for ch in str.chars() {
            if ch == ' ' {
                match section {
                    _ => {}
                    4 => state.halfmove_clock = acc_str.parse::<usize>().unwrap(),
                    5 => state.fullmoves = acc_str.parse::<usize>().unwrap(),
                }
                acc_str = default();

                section += 1;
                continue;
            }

            let panic = || -> ! { panic!("Invalid FEN String!") };

            match section {
                // write down the pieces
                0 => state.FEN_placement(ch, &mut square),
                // who's g dang turn is it??
                1 => {
                    state.turn = match ch {
                        'b' => false,
                        'w' => true,
                        _ => panic(),
                    }
                }
                // castling rights
                2 => match ch {
                    'q' => state.castling[0][0] = true,
                    'k' => state.castling[0][1] = true,
                    'Q' => state.castling[1][0] = true,
                    'K' => state.castling[1][1] = true,
                    '-' => {}
                    _ => panic(),
                },
                // en passant
                3 => {
                    if ch.is_ascii_digit() {
                        state.en_passant.push(std_position_to_pos(prev_char, ch));
                    }
                }
                // halfmove clock
                4 => acc_str.push(ch),
                // fullmove counter
                5 => acc_str.push(ch),
                _ => panic(),
            }
            prev_char = ch;
        }
        state.gen_threat();
        state.check_pins();
        state.move_gen();
        //dbg!(&state.moves);
        return state;
    }
}
