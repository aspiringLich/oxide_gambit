use std::cmp::max;

use bevy::prelude::default;

use crate::chess_logic::position::index_to_coord;

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

const DEBUG: bool = true;

impl ChessState {
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
        let mut closest: [[Option<(usize, u8)>; 9]; 2] = [[None; 9]; 2];

        self.pinned_pieces = vec![PinType::None; self.pieces[self.turn as usize].len()];

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
                        *x = if team == self.turn() { (i, u8::MAX) } else { (i, max) };
                    // otherwise if theres nothing there update it
                    } else {
                        *item = Some((i, max));
                    }
                }
            }
        }

        if DEBUG {
            dbg!(self.king_position);
            eprintln!("Closest black pieces: ");
            for item in closest[0] {
                if let Some((index, _)) = item {
                    dbg!(self.pieces[0][index]);
                } else {
                    dbg!(Piece::default());
                }
            }
            eprintln!("Closest white pieces: ");
            for item in closest[1] {
                if let Some((index, _)) = item {
                    dbg!(self.pieces[1][index]);
                } else {
                    dbg!(Piece::default());
                }
            }
        }

        for item in closest[1]
            .iter()
            .zip(closest[0].iter())
            .enumerate()
            .filter(|(i, x)| x.0.is_some() && x.1.is_some())
        {
            let (i, (Some(closest_black), Some(closest_white))) = item else { unreachable!() };

            // the index / distance of the closest pieces
            let piece_index = [closest_white.0, closest_black.0];
            let distance = [closest_white.1, closest_black.1];

            // if theres a possible threat and the opposite team is closer
            if possible_threat(self.pieces[self.opp_turn()][piece_index[self.opp_turn()]].variant(), i)
                && distance[self.turn()] != u8::MAX // our closest person is not invalid
                && distance[self.opp_turn()] > distance[self.turn()]
            {
                dbg!(self.pieces[self.turn as usize][piece_index[self.turn as usize]]);

                // get the piece we would like the modify
                let turn = self.turn();

                use PinType::*;
                self.pinned_pieces[piece_index[turn]] = Pinned(index_to_coord(i));
            }
        }
    }
}
