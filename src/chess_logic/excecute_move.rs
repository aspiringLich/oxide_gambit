use std::mem::swap;

use bevy::prelude::*;

use crate::chess_logic::{
    piece_type::{PieceType, PieceVariant},
    Piece,
};

use super::{ChessMove, ChessState, Position};

pub const CASTLING_POS: [u8; 4] = [56, 63, 0, 7];

impl ChessState {
    pub fn remove_piece(&mut self, piece: Piece) {
        // remove the pieces targetted squares
        self.rem_threat_piece(piece);
        // remove the targetted piece from the vector
        let pieces = &mut self.pieces[!self.turn as usize];
        // if you panic here something went wrong with syncing board and piece vecs
        pieces.swap_remove(pieces.iter().position(|&p| p == piece).unwrap());
        self.board[piece.position.int()] = default();
    }

    /// change state with a chess move
    pub fn excecute_chess_move(&mut self, chess_move: ChessMove) {
        self.excecute_move(
            Piece::new(self.at(chess_move.origin), chess_move.origin),
            chess_move.target,
        );
    }

    pub fn check_endgame(&mut self) {
        use crate::PieceVariant::*;

        if !self.endgame {
            if self.queen == [0, 0] {
                self.endgame = true;
                return;
            }
            let mut flag = false;
            let pieces: &Vec<Piece> =
                unsafe { std::mem::transmute(&self.pieces[self.turn as usize]) };
            for piece in pieces {
                match piece.variant() {
                    Rook => return,
                    Bishop | Knight if !flag => flag = true,
                    Bishop | Knight if flag => return,
                    _ => {}
                }
            }
            self.endgame = true;
        }
    }

    /// Change state with a piece moving to a position
    pub fn excecute_move(&mut self, piece: Piece, pos: Position) {
        // loop {}
        use Option::None;
        use PieceVariant::*;

        let turn = self.turn();

        let mut promotion = false;
        self.en_passant = None;

        // things we may need to update for specific pieces
        match piece.variant() {
            King => {
                self.king_position[piece.team() as usize] = pos;
                self.castling[self.turn() << 1] = false;
                self.castling[(self.turn() << 1) | 1] = false;

                // try to do castling
            }
            Pawn => {
                let diff = (piece.position.0 as i8 - pos.0 as i8).abs();
                // if this was a double forward
                if diff == 16 {
                    self.en_passant = Some(piece.forward().unwrap());
                }
                // if this was a capture and its unoccupied, its en passant!!
                else if diff != 8 && !self.occupied(pos) {
                    let mut target = pos.try_to([(0, 1), (0, -1)][self.turn()]).unwrap();

                    // band aid?
                    if !self.occupied(target) {
                        target = pos.try_to([(0, 1), (0, -1)][!self.turn()]).unwrap();
                    }
                    let target_piece = Piece::new(self.at(target), target);

                    // dbg!(target_piece);
                    self.move_piece_threat(target_piece, pos);

                    //dbg!(&self);
                }
                // promotion
                else if pos.y() == [0, 7][self.turn()] {
                    promotion = true;
                }
            }
            _ => {}
        };

        // update Castling rights
        let pos_arr = [piece.position.0, pos.0];

        for (i, rook_pos) in CASTLING_POS.iter().enumerate() {
            if pos_arr.contains(&rook_pos) {
                self.castling[i] = false;
            }
        }

        // if this is a capture
        let mut remove = default();
        if self.occupied(pos) && self.capturable(pos) {
            remove = Piece::new(self.board[pos.int()], pos);
            // remove the pieces targetted squares
            self.rem_threat_piece(remove);
            // remove the targetted piece from the vector
            let pieces = &mut self.pieces[!self.turn as usize];
            // if you panic here something went wrong with syncing board and piece vecs
            pieces.swap_remove(pieces.iter().position(|&p| p == remove).unwrap());

            // extra things to do based on the piece type
            let team = remove.team() as usize;
            match remove.variant() {
                Queen => {
                    self.queen[team] -= 1;
                    self.check_endgame()
                }
                Rook => self.check_endgame(),
                _ => {}
            };
        }

        if piece.variant() == King {
            // check to make sure we dont need to move that rook too
            self.do_king_move(piece, pos);
        } else if promotion {
            self.move_piece_threat(piece, pos);
            let to_queen = Piece::new(self.at(pos), pos);
            self.add_threat_piece(Piece::new(PieceType(self.turn, Queen), pos));

            let mut iter = self.pieces[turn].iter_mut();
            let to_queen = iter.find(|&&mut x| x == to_queen).unwrap();
            to_queen.variant = PieceType(to_queen.team(), Queen);

            self.board[pos.int()] = to_queen.variant;
        } else {
            self.move_piece_threat(piece, pos);
        }

        self.turn = !self.turn;

        self.check_pins();
        self.move_gen();

        // run update evaluation function
        self.update_evaluation(
            piece,
            pos,
            if remove.variant() == PieceVariant::None { None } else { Some(remove) },
        )
    }
}
