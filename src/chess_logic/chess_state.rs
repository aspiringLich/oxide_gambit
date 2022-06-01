use bevy::prelude::default;

use std::fmt::Debug;

use super::{ChessMove, Piece, PieceType, PieceVariant, Position};

/// stores the state of the chessboard
pub struct ChessState {
    pub board: [PieceType; 64],  // board representation: square wise
    pub pieces: [Vec<Piece>; 2], // board representation: piece wise
    pub turn: bool,              // true for white's move, false for black
    pub moves: Vec<ChessMove>,
    // private values that shouldnt be
}

impl ChessState {
    pub const fn new() -> Self {
        use PieceVariant::*;
        ChessState {
            board: [PieceType(false, None); 64],
            // storing the team may be redundant but hey
            pieces: [vec![], vec![]],
            turn: true,
            moves: vec![],
        }
    }

    pub fn add_piece(&mut self, ch: char, square: u8) {
        let id = PieceType::from_char(ch);
        self.board[square as usize] = id.clone();
        self.pieces[id.team() as usize].push(Piece::new(id, Position(square)));
    }
}

impl Debug for ChessState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::display_piece::PIECE_CHAR;
        let mut out: String = default();

        let piece_char = |piece: PieceType| {
            if piece.variant() as usize > 0 {
                let ch = PIECE_CHAR[piece.variant() as usize - 1];
                format!("{} ", if piece.team() { ch.to_ascii_uppercase() } else { ch })
            } else {
                format!(". ")
            }
        };

        // print out board representation
        for i in 0..64 {
            let pos = Position(i);
            out += &piece_char(self.at(Position(pos.x() + 8 * (7 - pos.y()))));
            if i % 8 == 7 {
                out += "\n";
            }
        }

        // print out piece representation
        for piece in self.pieces[0].iter() {
            out += &piece_char(piece.variant);
        }
        out += "\n";
        for piece in self.pieces[1].iter() {
            out += &piece_char(piece.variant);
        }
        out += "\n";

        // print out moves
        for m in self.moves.iter() {
            out += &format!("{} => {}\n", m.origin.int(), m.target.int());
        }

        // print out pieces
        f.write_str(&out)
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
