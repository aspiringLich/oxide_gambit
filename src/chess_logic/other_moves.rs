use crate::chess_logic::pin::PinType;

use super::{
    move_gen::ChessMove, piece, ChessState, Piece, PieceType, PieceVariant, Position, CASTLING_POS,
};
use bevy::prelude::*;

impl ChessState {
    /// scan these squares for castling availability
    pub fn castling_scan(&self, pos: Position, dir: (i8, i8)) -> bool {
        let mut itr = 1;
        while let Some(new_pos) = pos.try_to((dir.0 * itr, dir.1 * itr)) {
            if (self.occupied(new_pos) || self.threat_at(new_pos, !self.turn) > 0)
                && !(new_pos.x() == 0 || new_pos.x() == 7)
            {
                eprint!("failed at");
                dbg!(new_pos);
                return false;
            }
            itr += 1;
        }
        return true;
    }

    /// generate moves for castling
    pub fn gen_castling(&mut self) {
        if self.checked() {
            return;
        }
        for i in 0..2 {
            if !self.castling[self.turn() * 2 + i] {
                return;
            }
            let dir = [(-1, 0), (1, 0)][i % 2];
            let king_pos = self.king(self.turn);
            if self.castling_scan(king_pos, dir) {
                let target = king_pos.try_to([(-2, 0), (2, 0)][i % 2]).unwrap();
                self.moves.push_front(ChessMove::new(king_pos, target));
            }
        }
    }

    /// moves a piece and updates threatened squares
    pub fn move_piece_threat(&mut self, piece: Piece, pos: Position) {
        // update threatenned squares
        self.update_threat(piece, pos);

        // move the thing there
        self.board[pos.int()] = self.board[piece.position.int()];
        self.board[piece.position.int()] = default();

        // update the pieces
        self.pieces[self.turn as usize].iter_mut().find(|&&mut p| p == piece).unwrap().position =
            pos;
    }

    /// moves a piece and does not update threatened squares
    pub fn move_piece(&mut self, piece: Piece, pos: Position) {
        // move the thing there
        self.board[pos.int()] = self.board[piece.position.int()];
        self.board[piece.position.int()] = default();

        // update the pieces
        self.pieces[self.turn as usize].iter_mut().find(|&&mut p| p == piece).unwrap().position =
            pos;
    }

    /// excecute the castle thing maybe if its valid
    pub fn do_king_move(&mut self, piece: Piece, to: Position) {
        let diff = piece.position.0 as i8 - to.0 as i8;

        let rook_target;
        let rook_pos;

        let dir;

        // queenside castle
        if diff == 2 {
            rook_pos = Position(CASTLING_POS[self.turn() * 2]);
            rook_target = to.try_to((1, 0)).unwrap();
            dir = (-1, 0);
        }
        // kingside castle
        else if diff == -2 {
            rook_pos = Position(CASTLING_POS[self.turn() * 2 + 1]);
            rook_target = to.try_to((-1, 0)).unwrap();
            dir = (1, 0);
        } else {
            self.move_piece_threat(piece, to);
            return;
        }

        // stuff
        let mut rook = Piece::new(PieceType(self.turn, PieceVariant::Rook), rook_pos);

        self.move_piece_threat(piece, to);
        self.threatened[self.turn()].squares[self.king(self.turn).int()] -= 1;
        self.move_piece_threat(rook, rook_target);
        // rook.position = rook_target;
        // self.add_threat_piece(rook);
    }
}
