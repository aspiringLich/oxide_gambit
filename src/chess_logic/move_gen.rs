use super::{ChessState, Direction, Piece, Pos};

/// enum for storing the attribute
#[derive(Debug)]
pub enum MoveAttribute {
    None,
    Capture,
    Check,
    Checkmate,
    Stalemate,
    Promotion,
}

/// struct for holding a chess move
///     start - starting position
///     end - ending position
#[derive(Debug)]
pub struct ChessMove {
    pub origin: Pos,
    pub target: Pos,
    pub attribute: MoveAttribute,
}

impl ChessMove {
    pub const fn new(from: Pos, to: Pos, attr: MoveAttribute) -> Self {
        ChessMove { origin: from, target: to, attribute: attr }
    }
}

impl ChessState {
    pub const fn occupied(&self, pos: Pos) -> bool {
        self.board[pos.0 as usize].0 != 0
    }

    pub const fn team(&self, pos: Pos) -> bool {
        self.board[pos.0 as usize].team()
    }

    pub const fn id(&self, pos: Pos) -> u8 {
        self.board[pos.0 as usize].piece_id()
    }

    pub fn move_gen(&mut self) {
        use Direction::*;

        for piece in self.pieces[self.turn as usize].clone() {
            let mut target = {
                match piece.variant() {
                    Pawn => self.gen_pawn_moves(piece),
                    Rook => self.gen_sliding(piece, vec![U, D, L, R]),
                    _ => panic!(),
                }
            };
            self.moves.append(&mut target);
        }
    }

    /// generate moves on a list of directions
    pub fn gen_sliding(&self, piece: Piece, movements: Vec<(i8, i8)>) -> Vec<ChessMove> {
        let mut out: Vec<ChessMove> = vec![];

        for movement in movements {
            out.append(&mut self.gen_sliding_dir(piece, movement));
        }
        return out;
    }

    /// generate all pieces in a direction
    pub fn gen_sliding_dir(&self, piece: Piece, direction: (i8, i8)) -> Vec<ChessMove> {
        use MoveAttribute::*;

        let mut i = 0;
        let mut o_pos = piece.pos;
        let mut out: Vec<ChessMove> = vec![];

        // if were at the edge already break
        if o_pos.check_edge(direction) {
            //dbg!(piece, i);
            return out;
        }

        // while we havent hit the edge
        loop {
            i += 1;
            let pos: Pos = o_pos.to(direction, i);
            println!("{:#?}", pos.0);

            // oh no this square isnt empty
            if self.occupied(pos) {
                // check for capture
                if self.team(pos) != piece.team() {
                    out.push(ChessMove::new(o_pos, pos, Capture));
                }
                //dbg!(pos, i);
                return out;
            }

            out.push(ChessMove::new(o_pos, pos, None));

            // if we are at the edge, break
            if pos.check_edge(direction) {
                //dbg!(pos, i);
                return out;
            }
        }
    }

    /// generate moves a pawn could take
    #[inline]
    pub fn gen_pawn_moves(&self, piece: Piece) -> Vec<ChessMove> {
        use super::Direction::*;
        use MoveAttribute::*;

        // TODO: add en passant later lol
        let mut dir = 1;
        let pos = piece.pos;

        let mut out: Vec<ChessMove> = vec![];

        if !piece.team() {
            dir = -1;
        }

        // if the square ahead of it is not occupied,
        if !self.occupied(pos.to(U, dir)) {
            // !! advance
            out.push(ChessMove::new(piece.pos, pos.to(U, dir), None));

            // move two squares
            if (piece.team() && pos.y() < 6)
                || (!piece.team() && pos.y() > 1) && !self.occupied(pos.to(U, dir * 2))
            {
                out.push(ChessMove::new(piece.pos, pos.to(U, dir * 2), None));
            }
        }
        // capture
        if self.occupied(pos.to(TL, dir)) && !self.team(pos.to(TL, dir)) {
            out.push(ChessMove::new(piece.pos, pos.to(TL, dir), None));
        }
        if self.occupied(pos.to(TR, dir)) && !self.team(pos.to(TR, dir)) {
            out.push(ChessMove::new(piece.pos, pos.to(TR, dir), None));
        }

        return out;
    }
}
