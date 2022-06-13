use bevy::prelude::*;

use crate::chess_logic::{piece_type::PieceVariant, Piece};

use super::{ChessMove, ChessState, Position};

impl ChessState {
    /// Change state
    pub fn excecute_move(&mut self, piece: Piece, pos: Position) {
        use PieceVariant::*;

        // things we may need to update for specific pieces
        match piece.variant() {
            King => self.king_position[piece.team() as usize] = pos,
            _ => {}
        };

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

        // update threatenned squares
        self.update_threat(piece, pos);

        // move the thing there
        self.board[pos.int()] = self.board[piece.position.int()];
        self.board[piece.position.int()] = default();

        // update the pieces
        self.pieces[self.turn as usize].iter_mut().find(|&&mut p| p == piece).unwrap().position =
            pos;
        self.turn = !self.turn;
    }
}
