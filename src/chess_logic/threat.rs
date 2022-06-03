use std::{cmp::max, ops::Index};

use bevy::prelude::default;

use crate::chess_logic::PieceVariant;

use super::{ChessState, Moves, Piece, PieceType, Position};

#[derive(Debug)]
pub struct Threat {
    pub squares: [u8; 64],
}

impl Default for Threat {
    fn default() -> Self {
        Self { squares: [0; 64] }
    }
}

impl ChessState {
    /// remove threats from a range of positions
    /// assumes that the positions are diagonal / orthogonal from each other
    /// includes new_pos, does not include pos
    fn remove_from_pos_range(&mut self, mut pos: Position, new_pos: Position, piece: Piece) {
        let (x, y) = pos.rel_from(new_pos);
        let mut itr = 1;
        // we actually got blocked off by the piece oh noess
        while pos != new_pos {
            pos.modify(x * itr + y * 8 * itr);
            self.rem_threat(piece, pos);
            itr += 1;
        }
    }
    /// do before updating state
    pub fn update_threat(&mut self, piece: Piece, new_pos: Position) {
        use PieceVariant::*;

        self.rem_threat_piece(piece);
        self.add_threat_piece(Piece::new(piece.variant, new_pos));

        let pieces: &[Vec<Piece>; 2] = unsafe { std::mem::transmute(&self.pieces) };

        // returns a number from 0-8
        let coord_to_index = |(x, y): (i8, i8)| ((x.signum() + 1) + 3 * (y.signum() + 1)) as usize;
        let index_to_coord = |i: usize| -> (i8, i8) { (i as i8 / 3 - 1, i as i8 % 3 - 1) };

        // closest piece on each diagonal / orthagonal
        let mut closest_piece: [Piece; 9] = default();
        let mut distances: [i8; 9] = [i8::MAX; 9];

        // go through the pieces and find the closest ones
        for &new_piece in pieces[0].iter().chain(pieces[1].iter()) {
            let (x, y) = piece.rel_from(new_piece.position);
            if x == 0 || y == 0 || x.abs() == y.abs() {
                let max = if x == 0 { y.abs() } else { x.abs() };
                let index = coord_to_index((x, y));
                if max < distances[index] {
                    distances[index] = max;
                    closest_piece[index] = new_piece;
                }
            }
        }
        // reset middle
        closest_piece[4] = default();

        // for each closest piece, check if its valid,
        // if it satisfies criteria, remove all its threatenned squares in that direction and regenerate
        for (i, check_piece) in closest_piece.iter().enumerate() {
            let variant = check_piece.variant();
            if variant == Queen
                || (variant == Rook && i % 2 == 1)
                || (variant == Bishop && i % 2 == 0)
            {
                eprintln!("gaming");
                self.remove_from_pos_range(check_piece.position, piece.position, *check_piece);
                self.add_threat_dir(*check_piece, index_to_coord(i));
            }
        }
    }

    pub fn threat_at(&self, pos: Position, team: bool) -> u8 {
        self.threatened[team as usize].squares[pos.int()]
    }

    /// add threatenned squares
    pub fn gen_threat(&mut self) {
        let pieces: &[Vec<Piece>; 2] = unsafe { std::mem::transmute(&self.pieces) };

        for &piece in pieces[0].iter().chain(pieces[1].iter()) {
            self.add_threat_piece(piece);
        }
    }

    /// gen threatenned squares for one piece
    pub fn add_threat_piece(&mut self, piece: Piece) {
        use Moves::*;
        use PieceVariant::*;

        match piece.variant() {
            None => panic!("tried to generate threatenned squares of an invalid piece aaaa"),
            // sliding pieces
            Rook => self.add_threat_sliding(piece, RookMoves.get()),
            Bishop => self.add_threat_sliding(piece, BishopMoves.get()),
            Queen => self.add_threat_sliding(piece, QueenMoves.get()),
            // static pieces
            Pawn => {
                self.add_threat_static(piece, [PawnBMobes, PawnWMoves][piece.team() as usize].get())
            }
            Knight => self.add_threat_static(piece, KnightMoves.get()),
            King => self.add_threat_static(piece, QueenMoves.get()),
        }
    }

    /// add threatened squares in specified directions
    #[inline]
    fn add_threat_sliding(&mut self, piece: Piece, movements: &Vec<(i8, i8)>) {
        for movement in movements {
            self.add_threat_dir(piece, *movement);
        }
    }

    /// add threatened squares in set locations
    #[inline]
    fn add_threat_static(&mut self, piece: Piece, movements: &Vec<(i8, i8)>) {
        for movement in movements {
            if let Some(pos) = piece.try_to(*movement) {
                self.add_threat(piece, pos);
            }
        }
    }

    #[inline]
    fn add_threat(&mut self, piece: Piece, pos: Position) {
        self.threatened[piece.team() as usize].squares[pos.int()] += 1;
    }

    /// add threatenned squares in one direction
    #[inline]
    fn add_threat_dir(&mut self, piece: Piece, movement: (i8, i8)) {
        while let Some(pos) = piece.position.try_to(movement) {
            self.add_threat(piece, pos);
            if self.occupied(pos) {
                return;
            }
        }
    }
}

impl ChessState {
    /// remove threatenned squares for one piece
    pub fn rem_threat_piece(&mut self, piece: Piece) {
        use Moves::*;
        use PieceVariant::*;

        match piece.variant() {
            None => panic!("tried to remove threatenned squares of an invalid piece aaaa"),
            // sliding pieces
            Rook => self.rem_threat_sliding(piece, RookMoves.get()),
            Bishop => self.rem_threat_sliding(piece, BishopMoves.get()),
            Queen => self.rem_threat_sliding(piece, QueenMoves.get()),
            // static pieces
            Pawn => {
                self.rem_threat_static(piece, [PawnBMobes, PawnWMoves][piece.team() as usize].get())
            }
            Knight => self.rem_threat_static(piece, KnightMoves.get()),
            King => self.rem_threat_static(piece, QueenMoves.get()),
        }
    }

    /// add threatened squares in specified directions
    #[inline]
    fn rem_threat_sliding(&mut self, piece: Piece, movements: &Vec<(i8, i8)>) {
        for movement in movements {
            self.rem_threat_dir(piece, *movement);
        }
    }

    /// add threatenned squares in set locations
    #[inline]
    fn rem_threat_static(&mut self, piece: Piece, movements: &Vec<(i8, i8)>) {
        for movement in movements {
            if let Some(pos) = piece.try_to(*movement) {
                self.rem_threat(piece, pos);
            }
        }
    }

    /// add threatenned squares in one direction
    #[inline]
    fn rem_threat_dir(&mut self, piece: Piece, movement: (i8, i8)) {
        while let Some(pos) = piece.position.try_to(movement) {
            self.rem_threat(piece, pos);
            if self.occupied(pos) {
                return;
            }
        }
    }

    #[inline]
    fn rem_threat(&mut self, piece: Piece, pos: Position) {
        self.threatened[piece.team() as usize].squares[pos.int()] -= 1;
    }
}
