use super::{ChessMove, Piece, PieceType, Pos};

/// stores the state of the chessboard
#[derive(Debug)]
pub struct ChessState {
    pub board: [PieceType; 64],  // board representation: square wise
    pub pieces: [Vec<Piece>; 2], // piece wise representation
    pub turn: bool,              // true for white's move, false for black
    pub moves: Vec<ChessMove>,
    // private values that shouldnt be
}

impl ChessState {
    pub const fn new() -> Self {
        ChessState {
            board: [PieceType::new(0); 64],
            // storing the team may be redundant but hey
            pieces: [vec![], vec![]],
            turn: true,
            moves: vec![],
        }
    }

    pub fn add_piece(&mut self, ch: char, square: u8) {
        let id = PieceType::from_char(ch);
        self.board[square as usize] = id.clone();
        self.pieces[id.team() as usize].push(Piece::new(Pos(square), id));
    }

    /// loads a FEN string into the board state
    pub fn from_FEN(str: &str) -> Self {
        let mut state: ChessState = Self::new();
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
        state.move_gen();
        dbg!(&state.moves);
        return state;
    }
}

// /// return the id of a piece from a character in a FEN string
// fn id_from_char(ch: char) -> u8 {
//     let piece = match ch {
//         'p' | 'P' => 1,
//         'r' | 'R' => 2,
//         'n' | 'N' => 3,
//         'b' | 'B' => 4,
//         'k' | 'K' => 5,
//         'q' | 'Q' => 6,
//         _ => 0,
//     };
//     let team = if ch as u8 > 'a' as u8 { 0x00 } else { 0x80 };

//     return piece | team;
// }
