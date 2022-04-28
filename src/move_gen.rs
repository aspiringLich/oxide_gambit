use crate::piece::*;
use crate::State;

/// struct for holding a chess move
///     start - starting position
///     end - ending position
#[derive(Debug)]
pub struct chess_move {
    pub piece: Piece,
    pub target: u8,
}

impl chess_move {
    pub const fn new(piece: Piece, target: u8) -> Self {
        chess_move { piece, target }
    }
}

impl State {
    pub const fn occupied(&self, pos: u8) -> bool {
        if self.board[pos as usize].0 == 0 {
            false
        } else {
            true
        }
    }

    pub const fn team(&self, pos: u8) -> bool {
        self.board[pos as usize].team()
    }

    #[inline]
    pub fn push_move(&mut self, piece: Piece, target: u8) {
        self.moves.push(chess_move::new(piece, target));
    }

    pub fn move_gen(&mut self, team: bool) {
        for piece in self.pieces[team as usize].clone() {
            match piece.id.piece_id() {
                1 => self.pawn_moves(piece),
                2 => self.sliding_straight(piece),
                _ => {} // maybe add a panic here after testing
            }
        }
    }

    /// generate moves a horizontally / vertically moving piece
    #[inline]
    pub fn sliding_straight(&mut self, piece: Piece) {
        //use Position::*;
        let mut i: i8 = 0;

        for n in 0..4 {
            i = 0;
            let mut pos = piece.pos;
            // while we havent hit the edge
            loop {
                i += 1;
                pos = piece.pos(hacky_workaround_there_is_a_better_way_of_doing_this(n, i));
                // if we are at the edge, break
                if matches!(if n >= 2 { pos % 8 } else { pos / 8 }, 0 | 7) {
                    dbg!(pos, i);
                    break;
                }
                // oh no this square isnt empty
                if self.occupied(pos) {
                    // check for capture
                    if self.team(pos) != piece.team() {
                        self.push_move(piece, pos);
                    }
                    dbg!(pos, i);
                    break;
                }
                self.push_move(piece, pos);
            }
        }
    }

    /// generate moves a pawn could take
    #[inline]
    pub fn pawn_moves(&mut self, piece: Piece) {
        use Position::*;
        // TODO: add en passant later lol
        // white

        let mut dir = 1;

        if !piece.team() {
            dir = -1;
        }

        // if the square ahead of it is not occupied,
        if !self.occupied(piece.pos(Up(dir))) {
            // !! advance
            self.moves.push(chess_move::new(piece, piece.pos(Up(dir))));

            // move two squares
            if matches!(piece.pos(Rank), 1 | 6) && !self.occupied(piece.pos(Up(dir * 2))) {
                // !! advance 2 electric boogaloo
                self.push_move(piece, piece.pos(Up(dir * 2)));
            }
        }
        // capture
        if self.occupied(piece.pos(DiagTL(dir))) && !self.team(piece.pos(DiagTL(dir))) {
            self.push_move(piece, piece.pos(DiagTL(dir)));
        }
        if self.occupied(piece.pos(DiagTR(dir))) && !self.team(piece.pos(DiagTR(dir))) {
            self.push_move(piece, piece.pos(DiagTR(dir)));
        }
    }
}
