use std::collections::BTreeMap;
use std::default::default;

use crossterm::style::Stylize;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::chess::index::Index;
use crate::chess::Team;
use crate::chess::{board::Board, direction::Direction};

use crate::rules::piece::Piece;

use crate::chess::square::Square;
use crate::rules::piece_info::PieceInfo;
use crate::state::board_state::BoardState;

use super::attack::{Attacked, SlidingAttacks};

/// A move from one square to another
#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub struct Move {
    pub piece: Index<Piece>,
    pub to: Square,
    // pub active: bool,
}

/// Stores the list of moves that can be made
#[derive(Debug, Default, Clone)]
pub struct Moves {
    // good_moves: Fx
    moves: FxHashSet<Move>,
    pub(in crate::move_gen) callbacks: FxHashMap<Square, Vec<Index<Piece>>>,
    pub(in crate::move_gen) attacked: Attacked,
}

#[ctor::ctor]
pub static BOARD: Board<Square> = {
    let mut board = Board::default();
    for i in 0..64 {
        board[Square(i)] = Square(i);
    }
    board
};

impl Moves {
    pub fn new() -> Self {
        Self { ..default() }
    }

    pub fn threat_at(&self, square: Square, team: Team) -> u8 {
        self.attacked[team].non_sliding[square]
    }

    pub fn sliding_threat_at(&self, square: Square, team: Team) -> SlidingAttacks {
        self.attacked[team].sliding[square]
    }

    /// Adds a piece's moves to itself
    pub fn add_piece(
        &mut self,
        idx: Index<Piece>,
        piece: &PieceInfo,
        board: &BoardState,
        pos: Square,
    ) {
        // let piece = board.get_info(idx).unwrap();
        self.add_normal_moves(board, idx, pos, piece.team);
        for &dir in &piece.attacks {
            self.insert_sliding(idx, piece.team, pos, dir, board, false);
        }
    }

    /// Generates the list of moves given a board state
    pub fn generate(board: &BoardState) -> Self {
        let mut moves = Self::new();

        for (i, &idx) in board.board().iter().enumerate() {
            let piece = board.get_info(idx);
            if let Some(piece) = piece {
                moves.add_piece(idx, piece, board, Square(i as u8));
            }
        }

        moves
    }

    /// Gets all the moves for a particular piece
    pub fn filter(&self, piece: Index<Piece>) -> impl Iterator<Item = &Move> {
        self.moves.iter().filter(move |m| m.piece == piece)
    }

    /// Inserts a sliding move into the list of moves
    ///
    /// # Arguments
    ///  - `piece`: The piece that is moving
    ///  - `team`: The team of the piece that is moving
    ///  - `square`: The square that the piece is moving from
    ///  - `dir`: The direction that the piece is moving in
    ///  - `board`: The board state
    ///  - `active`: Whether or not the move is active
    ///  - `inclusive`: Whether or not to include the starting square
    pub fn insert_sliding(
        &mut self,
        piece: Index<Piece>,
        team: Team,
        square: Square,
        dir: Direction,
        board: &BoardState,
        inclusive: bool,
    ) {
        // go through the squares in this direction
        for &square in BOARD
            .iter_direction(dir, square)
            .skip((!inclusive) as usize)
        {
            let idx = board.board()[square];
            if let Some(p) = board.get_info(idx) {
                // if the piece is on the other team, add it to the list of moves
                if p.team != team {
                    // eprintln!("{} {square}", board.get_info(piece).unwrap().ch);
                    self.insert_good(piece, square, team);
                    self.attacked[team].add_sliding(square, dir);
                }
                break;
            } else {
                // eprintln!("{} {square}", board.get_info(piece).unwrap().ch);
                self.insert(piece, square, team);
                self.attacked[team].add_sliding(square, dir);
            }
        }
    }

    /// Removes a sliding move from the list of moves
    ///
    /// # Arguments
    /// - `piece`: The piece that is moving
    /// - `team`: The team of the piece that is moving
    /// - `square`: The square that the piece is moving from
    /// - `dir`: The direction that the piece is moving in
    /// - `board`: The board state
    /// - `inclusive`: Whether or not to include the starting square
    pub fn remove_sliding(
        &mut self,
        piece: Index<Piece>,
        team: Team,
        square: Square,
        dir: Direction,
        board: &BoardState,
        inclusive: bool,
    ) {
        // go through the squares in this direction
        for &square in BOARD
            .iter_direction(dir, square)
            .skip((!inclusive) as usize)
        {
            let idx = board.board()[square];
            if let Some(p) = board.get_info(idx) {
                // if the piece is on the other team, remove it too
                if p.team != team {
                    debug_assert!(
                        self.remove(piece, square, team),
                        "Expected move to exist {:?} -> {:?}",
                        idx,
                        square
                    );
                    self.attacked[team].remove_sliding(square, dir);
                }
                break;
            } else {
                self.remove(piece, square, team);
                self.attacked[team].remove_sliding(square, dir);
            }
        }
    }

    /// Inserts a move into the list of moves
    pub fn insert(&mut self, idx: Index<Piece>, square: Square, team: Team) {
        let check = self.moves.insert(Move { piece: idx, to: square });
        debug_assert!(check, "Move already exists: {:?} -> {:?}\n", idx, square);
        self.attacked[team].inc(square);
    }

    /// Removes a move from the list of moves, returns whether the move was there to begin with
    pub fn remove(&mut self, idx: Index<Piece>, square: Square, team: Team) -> bool {
        self.attacked[team].dec(square);
        self.moves.remove(&Move { piece: idx, to: square })
    }

    /// Inserts a *good* move into the list of moves
    pub fn insert_good(&mut self, idx: Index<Piece>, square: Square, team: Team) {
        self.moves.insert(Move {
            piece: idx,
            to: square,
        });
        self.attacked[team].inc(square);
    }
    
    /// Removes a *good* move from the list of moves, returns whether the move was there to begin with
    pub fn remove_good(&mut self, idx: Index<Piece>, square: Square, team: Team) -> bool {
        self.attacked[team].dec(square);
        self.moves.remove(&Move {
            piece: idx,
            to: square,
        })
    }
    
    pub fn insert_threat(&mut self, idx: Index<Piece>, square: Square, team: Team) {
        self.attacked[team].inc(square);
    }
    
    pub fn remove_threat(&mut self, idx: Index<Piece>, square: Square, team: Team) {
        self.attacked[team].dec(square);
    }
    
    /// Inserts a callback
    pub fn insert_callback(&mut self, square: Square, idx: Index<Piece>) {
        match self.callbacks.get_mut(&square) {
            Some(callbacks) => callbacks.push(idx),
            None => {
                self.callbacks.insert(square, vec![idx]);
            }
        }
    }
    
    /// Removes a callback
    pub fn remove_callback(&mut self, square: Square, idx: Index<Piece>) {
        match self.callbacks.get_mut(&square) {
            Some(callbacks) => {
                let idx = callbacks.iter().position(|&x| x == idx).unwrap();
                callbacks.remove(idx);
            }
            None => {
                panic!("Callback does not exist");
            }
        }
    }
}

/// Represents a difference between two lists of moves
struct Diff {
    /// The moves that should be there
    should: Vec<Square>,
    /// The moves that are there
    is: Vec<Square>,
}

enum DiffItem {
    Extra(Square),
    Missing(Square),
    Proper(Square),
}

impl DiffItem {
    fn get(&self) -> Square {
        *match self {
            DiffItem::Extra(s) => s,
            DiffItem::Missing(s) => s,
            DiffItem::Proper(s) => s,
        }
    }
}

impl Diff {
    fn items(self) -> Vec<DiffItem> {
        let mut out = Vec::new();
        for &should in &self.should {
            if self.is.contains(&should) {
                out.push(DiffItem::Proper(should));
            } else {
                out.push(DiffItem::Missing(should));
            }
        }
        for is in self.is {
            if !self.should.contains(&is) {
                out.push(DiffItem::Extra(is));
            }
        }
        out.sort_by(|a, b| {
            let a = a.get();
            let b = b.get();
            a.cmp(&b)
        });
        out
    }
}

/// Generates a list of differences between what the moves should be and what they are
fn generate_diff(
    this: &FxHashSet<Move>,
    other: &FxHashSet<Move>,
    state: &BoardState,
) -> BTreeMap<Index<Piece>, Diff> {
    let mut map = BTreeMap::new();
    let this = this.iter().collect::<Vec<_>>();
    let other = other.iter().collect::<Vec<_>>();

    for idx in state
        .pieces()
        .iter()
        .skip(1)
        .enumerate()
        .map(|(i, _)| Index::new(i as u8))
    {
        // get all the pieces moves into a list
        let get = |moves: &Vec<&Move>, piece| {
            let mut out = moves
                .iter()
                .filter(|m| m.piece == piece)
                .map(|m| m.to)
                .collect::<Vec<_>>();
            out.sort();
            out
        };
        // get the moves & the moves that should be there
        let is = get(&this, idx);
        let should = get(&other, idx);

        map.insert(idx, Diff { should, is });
    }

    map
}

impl Moves {
    pub fn fmt(&self, state: &BoardState, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let diff = generate_diff(&self.moves, &Moves::generate(state).moves, state);

        for (idx, diff) in diff {
            // let piece = state.get_idx(idx).expect("Only valid pieces have moves");
            let Some(piece) = state.get_info(idx) else { continue };
            let mut extra = String::new();
            let mut missing = String::new();
            let mut proper = String::new();

            for item in diff.items() {
                let c = |s: Square| (String::from(" ") + &s.to_string());
                match item {
                    DiffItem::Extra(s) => extra += &c(s),
                    DiffItem::Missing(s) => missing += &c(s),
                    DiffItem::Proper(s) => proper += &c(s),
                }
            }
            let add_line = |s: &mut String, prefix: &str| {
                if !s.is_empty() {
                    *s = format!("{prefix}{s}{}", '\n')
                }
            };
            add_line(&mut extra, "  + ");
            add_line(&mut missing, "  - ");
            add_line(&mut proper, &format!(" {}{}:", idx, piece.ch));

            match piece.team {
                Team::Black => write!(f, "{}", proper.green())?,
                Team::White => write!(f, "{}", proper.blue())?,
            }
            write!(f, "{}{}", missing.red(), extra.green())?;
        }

        Ok(())
    }
}
