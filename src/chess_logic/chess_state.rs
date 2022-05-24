use super::{ChessMove, Piece, PieceType, Pos};

/// stores the state of the chessboard
#[derive(Debug)]
pub struct ChessState {
    pub board: [PieceType; 64],  // board representation: square wise
    pub pieces: [Vec<Piece>; 2], // board representation: piece wise
    pub turn: bool,              // true for white's move, false for black
    pub moves: Vec<ChessMove>,
    // private values that shouldnt be
}

impl ChessState {
    pub const fn new() -> Self {
        ChessState {
            board: [Default::default(); 64],
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
