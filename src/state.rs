

/// enum for holding a Piece


/// return the id of a piece from a character in a FEN string
fn id_from_char(ch: char) -> u8 {
    let piece = match ch {
        'p' | 'P' => 1,
        'r' | 'R' => 2,
        'n' | 'N' => 3,
        'b' | 'B' => 4,
        'k' | 'K' => 5,
        'q' | 'Q' => 6,
        _ => 0,
    };
    let team = if ch as u8 > 'a' as u8 { 0x80 } else { 0x00 };

    return piece | team;
}

/// stores the state of the chessboard
#[derive(Debug)]
pub struct State {
    pub board: [u8; 64], // board representation: square wise
    pub pieces: Vec<u8>,
    turn: bool,      // true for white's move, false for black
}

impl State {
    pub fn new() -> Self {
        State {
            board: [0; 64],
            pieces: vec![],
            turn: true,
        }
    }

    /// loads a FEN string into the board state
    pub fn from_FEN(str: &String) -> Self {
        let mut state: State = Self::new();
        let mut section = 0; // which section of the FEN string are we on?

        // 0    => pieces on the board  "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"
        // 1    => turn                 "w"
        // 2    => castling rights      "KQkq"
        // 3    => en passant?          "-"
        // 4    => halfmove clock       "0"
        // 5    => move counter         "1"

        let mut i = 0; // square number you are on
        for ch in str.chars() {
            if ch == ' ' {
                section += 1;
                continue;
            }

            //print!("{} ", i);
            match section {
                0 => {
                    state.board[i - 1] = match ch {
                        '1'..='9' => {
                            i += ch as usize - '0' as usize;
                            0
                        }
                        '/' => continue,
                        _ => {
                            i += 1;
                            id_from_char(ch)
                        }
                    };
                }
                1 => {
                    if ch == 'b' {
                        state.turn = false;
                    }
                }
                2 => {}
                3 => {}
                4 => {}
                5 => {}
                _ => unreachable!(), // if it gets here, uh, invalid FEN string i guess
            }
        }
        return state;
    }
}

// enum Piece {
//     Pawn,
//     Rook,
//     Knight,
//     Bishop,
//     King,
//     Queen,
//     None,
// }

// enum Team {
//     White,
//     Black,
// }

// impl Piece {
//     pub fn id(&self, team: &Team) -> u8 {
//         return match self {
//             Piece::Pawn => 1,
//             Piece::Rook => 2,
//             Piece::Knight => 3,
//             Piece::Bishop => 4,
//             Piece::King => 5,
//             Piece::Queen => 6,
//             Piece::None => return 0,
//         } | match team {
//             Team::White => 0x00,
//             Team::Black => 0x80
//         };
//     }
// }
