use crate::piece::*;

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
    let team = if ch as u8 > 'a' as u8 { 0x00 } else { 0x80 };

    return piece | team;
}

/// stores the state of the chessboard
#[derive(Debug)]
pub struct State {
    pub board: [PieceType; 64], // board representation: square wise
    pub white_pieces: Vec<Piece>,
    pub black_pieces: Vec<Piece>,
    turn: bool, // true for white's move, false for black
}

impl State {
    pub fn new() -> Self {
        State {
            board: [PieceType(0); 64],
            // storing the team may be redundant but hey
            white_pieces: vec![],
            black_pieces: vec![],
            turn: true,
        }
    }

    /// loads a FEN string into the board state
    pub fn from_FEN(str: &str) -> Self {
        let mut state: State = Self::new();
        let mut section = 0; // which section of the FEN string are we on?

        // 0    => pieces on the board  "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"
        // 1    => turn                 "w"
        // 2    => castling rights      "KQkq"
        // 3    => en passant?          "-"
        // 4    => halfmove clock       "0"
        // 5    => move counter         "1"

        let mut square = 0; // square number you are on
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
                        '1'..='8' => {
                            square += ch as usize - '0' as usize;
                        }
                        // end of a rank
                        '/' => continue,
                        // found something else whee
                        _ => {
                            square += 1;

                            let id = PieceType::from_char(ch);
                            state.board[square - 1] = id.clone();
                            if id.team() {
                                // white
                                state.white_pieces.push(Piece::new(square as u8 - 1, id));
                            }
                            else {
                                //black
                                state.black_pieces.push(Piece::new(square as u8 - 1, id));
                            }
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
        return state;
    }
}
