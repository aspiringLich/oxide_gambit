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

    /// Change state
    pub fn excecute_move(&mut self, piece: Piece, pos: Position) {
        use Option::None;
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
                self.en_passant = None;
                let diff = (piece.position.0 as i8 - pos.0 as i8).abs();
                // if this was a double forward
                if diff == 16 {
                    self.en_passant = Some(piece.forward().unwrap());
                }
                // if this was a capture and its unoccupied, its en passant!!
                else if diff != 8 && !self.occupied(pos) {
                    let target = pos.try_to([(0, 1), (0, -1)][self.turn()]).unwrap();
                    let target_piece = Piece::new(self.at(target), target);
                    dbg!(target_piece);
                    self.move_piece_threat(target_piece, pos);
                    //dbg!(&self);
                }
                // promotion
                else if pos.y() == [0, 7][self.turn()] {
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
