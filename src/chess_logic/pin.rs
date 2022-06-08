use std::cmp::max;

use bevy::prelude::default;

use super::{
    chess_state::ChessState,
    position::{coord_to_index, is_45, Position},
    threat::possible_threat,
    Piece,
};

#[derive(Debug, Clone)]
pub enum PinType {
    Pinned,           // a piece is completely pinned
    PinDir((u8, u8)), // a piece can still move along this direction (and its inverse)
    None,             // a piece is not pinned
}

impl Default for PinType {
    fn default() -> Self {
        PinType::None
    }
}

impl ChessState {
    pub fn king(&self, turn: bool) -> Position {
        self.king_position[self.turn as usize]
    }

    pub fn check_pins(&mut self) {
        let pieces: &[Vec<Piece>; 2] = unsafe { std::mem::transmute(&self.pieces) };

        // closest pieces
        let mut closest: [[Option<(usize, i8)>; 9]; 2] = [[None; 9]; 2];

        self.pinned_pieces = vec![PinType::None; self.pieces[self.turn as usize].len()];

        // find closest pieces for each team
        for team in 0..=1 {
            for (i, piece) in pieces[team].iter().enumerate() {
                let (x, y) = piece.rel_from(self.king(self.turn));
                if is_45(x, y) {
                    let max = max(x.abs(), y.abs());
                    let index = coord_to_index(x, y);

                    let item: &mut Option<(usize, i8)> = &mut closest[team][coord_to_index(x, y)];
                    // if theres something there already and its further away, update it
                    if let Some(x) = item {
                        if x.1 > max {
                            *x = (i, max)
                        }
                    // otherwise if theres nothing there update it
                    } else {
                        *item = Some((i, max));
                    }
                }
            }
        }

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

        for item in closest[self.turn as usize]
            .iter()
            .zip(closest[!self.turn as usize].iter())
            .enumerate()
            .filter(|(i, x)| x.0.is_some() && x.1.is_some())
        {
            let (i, (Some(closest_white), Some(closest_black))) = item else { unreachable!() };

            // the index / distance of the closest pieces
            let piece_index = [closest_white.0, closest_black.0];
            let distance = [closest_white.1, closest_black.1];

            eprintln!("Closest piece pairs:");
            dbg!(self.pieces[0][piece_index[!self.turn as usize]]);
            dbg!(self.pieces[1][piece_index[self.turn as usize]]);

            if possible_threat(
                self.pieces[!self.turn as usize][piece_index[self.turn as usize]].variant(),
                i,
            ) {
                dbg!(self.pieces[!self.turn as usize][piece_index[self.turn as usize]], i);
            }
        }
    }
}
