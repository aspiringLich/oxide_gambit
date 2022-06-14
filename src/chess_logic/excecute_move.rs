use std::mem::swap;

use bevy::prelude::*;

use crate::chess_logic::{piece_type::PieceVariant, Piece};

use super::{ChessMove, ChessState, Position};

pub const CASTLING_POS: [u8; 4] = [56, 63, 0, 7];

impl ChessState {
    /// Change state
    pub fn excecute_move(&mut self, piece: Piece, pos: Position) {
        use PieceVariant::*;

        // things we may need to update for specific pieces
        match piece.variant() {
            King => {
                self.king_position[piece.team() as usize] = pos;
                self.castling[self.turn() << 1] = false;
                self.castling[(self.turn() << 1) | 1] = false;

                // try to do castling
            }
            Pawn => {
                let turn = self.turn();

                // if this was a double forward
                if (piece.position.0 as i8 - pos.0 as i8).abs() == 16 {
                    self.en_passant.push(piece.forward().unwrap());
                } else {
                    // remove en passant
                    if piece.y() == [4, 3][self.turn()] {
                        let rmv = self
                            .en_passant
                            .iter_mut()
                            .position(|&mut x| x == piece.backward().unwrap());
                        // dbg!(piece_backward());
                        if let Some(rmv) = rmv {
                            self.en_passant.swap_remove(rmv);
                        }
                    }
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
        if self.occupied(pos) && self.capturable(pos) {
            let remove = Piece::new(self.board[pos.int()], pos);
            // remove the pieces targetted squares
            self.rem_threat_piece(remove);
            // remove the targetted piece from the vector
            let pieces = &mut self.pieces[!self.turn as usize];
            // if you panic here something went wrong with syncing board and piece vecs
            pieces.swap_remove(pieces.iter().position(|&p| p == remove).unwrap());

            if remove.variant() == Pawn && pos.y() == [4, 3][self.opp_turn()] {
                let rmv =
                    self.en_passant.iter_mut().position(|&mut x| x == remove.backward().unwrap());
                if let Some(rmv) = rmv {
                    self.en_passant.swap_remove(rmv);
                }
            }
        }

        if piece.variant() == King {
            // check to make sure we dont need to move that rook too
            self.do_king_move(piece, pos);
        } else {
            self.move_piece_threat(piece, pos);
        }

        self.turn = !self.turn;

        self.check_pins();
        self.move_gen();
    }
}
