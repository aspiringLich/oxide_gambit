use super::{Piece, Position, State};

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
                1 => self.gen_pawn_moves(piece),
                2 => self.gen_sliding_straight(piece),
                _ => {} // maybe add a panic here after testing
            }
        }
    }

    /// generate moves a horizontally / vertically moving piece
    #[inline]
    pub fn gen_sliding_straight(&mut self, piece: Piece) {
        use Position::*;
        let mut i: i8 = 0;

        let positions: [Position; 4] = [Up(1), Down(1), Left(1), Right(1)];

        for n in 0..4 {
            self.gen_sliding_dir(piece, positions[n])
        }
    }

    /// generate all pieces in a direction
    #[inline]
    pub fn gen_sliding_dir(&mut self, piece: Piece, position: Position) {
        let mut i = 0;
        let mut pos;

        // if were at the edge already break
        if check_edge(position, piece) {
            dbg!(piece.pos, i);
            return;
        }

        // while we havent hit the edge
        loop {
            i += 1;
            pos = piece.mutate(position).pos;
            // if we are at the edge, break
            if check_edge(position, piece) {
                dbg!(piece.pos, i);
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

    /// generate moves a pawn could take
    #[inline]
    pub fn gen_pawn_moves(&mut self, piece: Piece) {
        use super::Position::*;
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

fn check_edge(position: Position, piece: Piece) -> bool {
    use Position::*;

    match position {
        Up(_) => piece.pos(Rank) == 0,
        Down(_) => piece.pos(Rank) == 7,
        Left(_) => piece.pos(File) == 0,
        Right(_) => piece.pos(File) == 7,
        DiagTL(_) => piece.pos(Rank) == 0 || piece.pos(File) == 0,
        DiagTR(_) => piece.pos(Rank) == 0 || piece.pos(File) == 7,
        DiagBL(_) => piece.pos(Rank) == 7 || piece.pos(File) == 0,
        DiagBR(_) => piece.pos(Rank) == 7 || piece.pos(File) == 7,
        _ => unimplemented!(),
    }
}
