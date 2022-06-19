use std::cmp::max;

use bevy::prelude::default;

use crate::chess_logic::{piece_type::PieceType, position::index_to_coord};

use super::{
    chess_state::ChessState,
    position::{coord_to_index, is_45, Position},
    threat::possible_threat,
    Piece,
};

#[derive(Debug, Clone, PartialEq)]
pub enum PinType {
    Pinned((i8, i8)), // a piece can still move along this direction (and its inverse)
    None,             // a piece is not pinned
}

impl Default for PinType {
    fn default() -> Self {
        PinType::None
    }
}

const DEBUG: bool = false;

impl ChessState {
    pub fn checked(&self) -> bool {
        self.threatened[self.opp_turn()].squares[self.king_position[self.turn()].0 as usize] > 0
    }

    pub fn opp_turn(&self) -> usize {
        !self.turn as usize
    }

    pub fn turn(&self) -> usize {
        self.turn as usize
    }

    pub fn king(&self, turn: bool) -> Position {
        self.king_position[self.turn as usize]
    }

    pub fn check_pins(&mut self) {
        let pieces: &[Vec<Piece>; 2] = unsafe { std::mem::transmute(&self.pieces) };

        // closest pieces
        let mut closest: [[Option<(usize, u8)>; 9]; 2] = [[Option::None; 9]; 2];
        let mut valid: [bool; 9] = [true; 9];

        self.pinned_pieces = vec![PinType::None; self.pieces[self.turn as usize].len()];
        self.constraint = Option::None;

        let king_pos = self.king_position[self.turn()];

        let mut knight_index = usize::MAX;

        // find closest pieces for each team
        for team in 0..=1 {
            for (i, piece) in pieces[team].iter().enumerate() {
                let (x, y) = piece.rel_from(self.king(self.turn));
                if is_45(x, y) {
                    let max = max(x.abs() as u8, y.abs() as u8);
                    let index = coord_to_index(x, y);

                    let item: &mut Option<(usize, u8)> = &mut closest[team][coord_to_index(x, y)];
                    // if theres something there already set the distance to max to show its invalid
                    // if its our own guy
                    if let Some(x) = item {
                        if self.turn() == i {
                            valid[i] = false;
                        }
                        *x = [(i, max), *x][(max > x.1) as usize];
                    // otherwise if theres nothing there update it
                    } else {
                        *item = Some((i, max));
                    }
                } else if piece.variant() == Knight
                    && piece.team() != self.turn
                    && self.checked()
                    && KnightMoves.get().contains(&piece.position.rel_from(self.king(self.turn)))
                {
                    knight_index = i;
                }
            }
        }

        if DEBUG {
            dbg!(self.king_position);
            eprintln!("Closest black pieces: ");
            for item in closest[0] {
                if let Some((index, i)) = item {
                    eprint!("{} ", i);
                    dbg!(self.pieces[0][index]);
                } else {
                    dbg!(Piece::default());
                }
            }
            eprintln!("Closest white pieces: ");
            for item in closest[1] {
                if let Some((index, i)) = item {
                    eprint!("{} ", i);
                    dbg!(self.pieces[1][index]);
                } else {
                    dbg!(Piece::default());
                }
            }
        }

        let zip = closest[0].iter().zip(closest[1].iter()).enumerate();

        use crate::move_gen::Moves::*;
        use crate::PieceVariant::*;

        if self.checked() {
            // dbg!(zip.clone().collect::<Vec<_>>());
            // go through the items if theres only the other team
            let other_only = zip
                .clone()
                .filter(|(i, x)| {
                    let x = [x.0, x.1];
                    ((x[self.turn()].is_none() || x[self.turn()].unwrap().1 == u8::MAX)
                        && x[self.opp_turn()].is_some())
                })
                .map(|(i, x)| (i, [x.1, x.0][self.turn()]));
            if DEBUG {
                dbg!(other_only.clone().collect::<Vec<_>>());
            }

            let mut itr = 0;
            let mut other_squares: Vec<Position> = vec![];

            // if were not being threatenned by a knight...
            if knight_index == usize::MAX {
                for item in other_only {
                    if let (i, Some(item)) = item {
                        let piece = self.pieces[self.opp_turn()][item.0];
                        let pos = piece.position;
                        let check_threat = || possible_threat(piece.variant(), i);

                        if check_threat() {
                            if DEBUG {
                                eprint!("threat found whee");
                                dbg!(piece);
                            }
                            if itr == 0 {
                                other_squares = self
                                    .gen_sliding_dir_pos(piece, index_to_coord(i))
                                    .unwrap_or(vec![]);
                                if other_squares.is_empty() {
                                    continue;
                                }
                                other_squares.push(pos);
                            }
                            itr += 1;
                        // if its a knight or pawn and is threatening king
                        } else if (piece.variant() == Pawn
                            && [PawnWMoves, PawnBMoves][self.turn()]
                                .get()
                                .contains(&pos.rel_from(king_pos)))
                        {
                            if DEBUG {
                                eprintln!("pawn moment")
                            }
                            itr += 1;
                            other_squares.push(pos);
                        }
                    }
                }
                // if theres one piece thats a threat,
                if itr == 1 {
                    other_squares.sort_by(|&a, &b| a.0.cmp(&b.0));
                    self.constraint = Some(other_squares);
                } else if itr > 1 {
                    self.constraint = Some(vec![]);
                }
            // knight moment
            } else {
                if DEBUG {
                    eprintln!("knight moment")
                }
                self.constraint = Some(vec![self.pieces[self.opp_turn()][knight_index].position]);
            }

            if DEBUG {
                dbg!(itr);
                dbg!(&self.constraint);
            }
        }

        // go through the items if both are some
        for item in zip.filter(|(i, x)| (x.0.is_some() && x.1.is_some())) {
            let (i, (Some(closest_black), Some(closest_white))) = item else { unreachable!() };

            // the index / distance of the closest pieces
            let piece_index = [closest_black.0, closest_white.0];
            let distance = [closest_black.1, closest_white.1];
            let dir = index_to_coord(8 - i);

            // find the next piece after this
            let our_piece = self.pieces[self.turn()][piece_index[self.turn()]];
            let mut itr = 1;
            let mut try_pos = our_piece.try_to((dir.0 * itr, dir.1 * itr));
            while try_pos.is_some() && !self.occupied(try_pos.unwrap()) {
                itr += 1;
                try_pos = our_piece.try_to((dir.0 * itr, dir.1 * itr));
            }
            let their_piece: PieceType = match try_pos {
                Some(n) => self.at(n),
                _ => self.at(our_piece.try_to((dir.0 * (itr - 1), dir.1 * (itr - 1))).unwrap()),
            };

            if DEBUG {
                dbg!(their_piece);
            }

            // if theres a possible threat and the opposite team is closer
            if possible_threat(their_piece.variant(), i)
                && distance[self.opp_turn()] > distance[self.turn()]
                && their_piece.team() != self.turn
            {
                if DEBUG {
                    dbg!(self.pieces[self.turn as usize][piece_index[self.turn as usize]]);
                }

                // get the piece we would like the modify
                let turn = self.turn();

                use PinType::*;
                self.pinned_pieces[piece_index[turn]] = Pinned(index_to_coord(i));
            }
        }
    }
}
