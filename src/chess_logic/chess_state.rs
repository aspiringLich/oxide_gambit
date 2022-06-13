use bevy::prelude::default;

use std::{collections::VecDeque, fmt::Debug};

use super::{pin::PinType, ChessMove, Piece, PieceType, PieceVariant, Position, Threat};

/// stores the state of the chessboard
pub struct ChessState {
    pub board: [PieceType; 64],            // board representation: square wise
    pub pieces: [Vec<Piece>; 2],           // board representation: piece wise
    pub turn: bool,                        // true for white's move, false for black
    pub castling: [[bool; 2]; 2],          // kingside / queenside castling rights
    pub en_passant: Vec<Position>,         // store the possible target squares for en passant
    pub halfmove_clock: usize, // halfmove counter - when it reaches 100 the game is drawn
    pub fullmoves: usize,      // number of times black has moved essentially
    pub moves: VecDeque<ChessMove>, // I GET TO USE A VECDEQUE also stores all the chess moves
    pub constraint: Option<Vec<Position>>, // if the king is under check, constrain the squares said king can go to
    pub threatened: [Threat; 2],           // which squares are under attack aaa
    pub king_position: [Position; 2],      // where 2 find kings
    pub pinned_pieces: Vec<PinType>,       // are any of the current pieces pinned
}

impl Default for ChessState {
    fn default() -> Self {
        Self {
            board: [default(); 64],
            // storing the team may be redundant but hey
            pieces: [vec![], vec![]],
            turn: true,
            castling: [[false; 2]; 2],
            en_passant: default(),
            halfmove_clock: 0,
            fullmoves: 1,
            moves: default(),
            constraint: default(),
            threatened: default(),
            king_position: default(),
            pinned_pieces: default(),
        }
    }
}

impl ChessState {
    pub fn add_piece(&mut self, ch: char, square: u8) {
        let id = PieceType::from_char(ch);
        self.board[square as usize] = id.clone();
        self.pieces[id.team() as usize].push(Piece::new(id, Position(square)));

        if id.variant() == PieceVariant::King {
            self.king_position[id.team() as usize] = Position(square);
        }
    }
}

impl Debug for ChessState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::display_piece::PIECE_CHAR;
        let mut out: String = default();

        let piece_char = |piece: PieceType| {
            if piece.variant() as usize > 0 {
                let ch = PIECE_CHAR[piece.variant() as usize - 1];
                format!("{}  ", if piece.team() { ch.to_ascii_uppercase() } else { ch })
            } else {
                format!(".  ")
            }
        };

        // print out board representation
        for i in 0..64 {
            if i % 8 == 0 {
                out += &format!("\n{:2}: ", (7 - (i / 8)) * 8);
            }
            let pos = Position(i);
            out += &piece_char(self.at(Position(pos.x() + 8 * (7 - pos.y()))));
        }
        out += "\n\n";

        out += "    White:                      Black:";
        // print out threatenned squares
        for i in 0..8 {
            out += &format!("\n{:2}: ", (7 - i) * 8);
            for j in 0..8 {
                let pos = Position((7 - i) * 8 + j);
                let threat = self.threat_at(pos, true);
                if threat > 0 {
                    out += &format!("{:<3}", threat)
                } else {
                    out += ".  "
                };
            }
            out += "    ";
            for j in 0..8 {
                let pos = Position((7 - i) * 8 + j);
                let threat = self.threat_at(pos, false);
                if threat > 0 {
                    out += &format!("{:<3}", threat)
                } else {
                    out += ".  "
                };
            }
        }
        out += "\n\n";

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
        for (i, m) in self.moves.iter().enumerate() {
            out += &format!(
                "{:16}",
                format!(
                    "({}) {} => {}",
                    &piece_char(self.at(m.origin)).chars().next().unwrap(),
                    m.origin.int(),
                    m.target.int()
                )
            );
            if i % 4 == 3 {
                out += "\n";
            }
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
