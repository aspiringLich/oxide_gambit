use super::{board::Board, moves::*, pieces::*, raycast::*, square::Square, state::*};

impl State {
    /// regenerate every legal move for every piece
    pub fn regenerate_moves(&mut self) {
        let board = &self.board;
        for (pos, piece) in board.iter().enumerate() {
            if piece.is_none() {
                continue;
            }

            let square = Square::new(pos as u8);
            let id = piece.get_id() as usize;

            let moves = self.moves[id].as_mut().unwrap();
            macro gen_line($d:expr) {
                generate_line(moves, board, square, *piece, $d);
            }

            macro try_add_capture(s: expr) {
                if board[*s as usize].is_none() {
                    moves.add_move(s, Priority::Low);
                } else {
                    if board[*s as usize].team() != Piece::Team {
                        moves.add_move(s, Priority::High);
                    }
                }
            };
            match piece.piece() {
                PAWN => {
                    let team = piece.team() as usize;
                    let dir = [-1, 1][team];

                    // forward moves
                    let new = square.try_to((0, dir)).unwrap();
                    if self.board[new].is_none() {
                        moves.add_move(new, Priority::Low);

                        let new = square.try_to((0, dir * 2)).unwrap();
                        if square.y() == [6, 1][team] && self.board[new].is_none() {
                            moves.add_move(new, Priority::High)
                        }
                    }
                }
                ROOK => {
                    gen_line!(Ray::Up);
                    gen_line!(Ray::Down);
                    gen_line!(Ray::Left);
                    gen_line!(Ray::Right);
                }
                KNIGHT => {}
                BISHOP => {
                    gen_line!(Ray::UpL);
                    gen_line!(Ray::DownL);
                    gen_line!(Ray::UpR);
                    gen_line!(Ray::DownR);
                }
                QUEEN => {
                    gen_line!(Ray::Up);
                    gen_line!(Ray::Down);
                    gen_line!(Ray::Left);
                    gen_line!(Ray::Right);
                    gen_line!(Ray::UpL);
                    gen_line!(Ray::DownL);
                    gen_line!(Ray::UpR);
                    gen_line!(Ray::DownR);
                }
                KING => {}
                _ => unreachable!(),
            }
        }
    }
}

/// generate a line of moves including the first occupied square if its of the other team
pub fn generate_line(
    moves: &mut Moves,
    board: &Board,
    start: Square,
    piece: Piece,
    direction: Ray,
) {
    let mut iter = Raycast::new(start, direction);
    // for every valid square in this direction
    while let Some(square) = iter.next() {
        // if occupied
        let at = board[square];
        if at.is_some() {
            if at.team() != piece.team() {
                moves.add_move(square, Priority::High)
            }
            return;
        }
        moves.add_move(square, Priority::Low)
    }
}
