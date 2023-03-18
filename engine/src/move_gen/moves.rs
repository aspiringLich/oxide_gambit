use std::collections::BTreeMap;
use std::default::default;
use std::fmt::Display;
use std::io::stdout;

use crossterm::style::Stylize;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::chess::board::Board;
use crate::chess::direction::Direction;
use crate::chess::index::Index;
use crate::chess::Team;
use crate::misc;
use crate::rules::piece::Piece;

use crate::chess::square::Square;
use crate::rules::piece_info::PieceInfo;
use crate::state::board_state::BoardState;

use super::attack::AttackedSquares;

/// A move from one square to another
#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub struct Move {
    pub piece: Index<Piece>,
    pub to: Square,
    // pub active: bool,
}

/// Stores the list of moves that can be made
#[derive(Debug, Default)]
pub struct Moves {
    moves: FxHashSet<Move>,
    callbacks: FxHashMap<Square, Vec<Index<Piece>>>,
    attacked: AttackedSquares,
}

impl Moves {
    pub fn new() -> Self {
        Self {
            ..default()
        }
    }

    /// Adds a piece's moves to itself
    pub fn add_piece(&mut self, idx: Index<Piece>, piece: &PieceInfo, board: &BoardState, pos: Square) {
        if let Some(moves) = piece.move_gen {
            moves(board, self, pos, piece.team);
        }
        for &dir in &piece.attacks {
            self.insert_sliding(idx, piece.team, pos, dir, board, false);
        }
    }

    /// Generates the list of moves given a board state
    pub fn generate(board: &BoardState) -> Self {
        let mut moves = Self::new();

        for (i, &idx) in board.board().iter().enumerate() {
            let piece = board.get_idx(idx);
            if let Some(piece) = piece {
                moves.add_piece(idx, piece, board, Square(i as u8));
            }
        }

        moves
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
        for &idx in board
            .board()
            .iter_direction(dir, square)
            .skip((!inclusive) as usize)
        {
            if let Some(p) = board.get_idx(idx) {
                // if the piece is on the other team, add it to the list of moves
                if p.team != team {
                    self.insert_good(piece, square);
                    self.attacked.add_sliding(square, dir);
                }
                break;
            } else {
                self.insert(piece, square);
                self.attacked.add_sliding(square, dir);
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
        for &idx in board
            .board()
            .iter_direction(dir, square)
            .skip((!inclusive) as usize)
        {
            if let Some(p) = board.get_idx(idx) {
                // if the piece is on the other team, remove it too
                if p.team != team {
                    debug_assert!(
                        self.remove(piece, square),
                        "Expected move to exist {:?} -> {:?}",
                        idx,
                        square
                    );
                    self.attacked.remove_sliding(square, dir);
                }
                break;
            } else {
                self.remove(piece, square);
                self.attacked.remove_sliding(square, dir);
            }
        }
    }

    /// Inserts a move into the list of moves
    pub fn insert(&mut self, piece: Index<Piece>, to: Square) {
        let check = self.moves.insert(Move { piece, to });
        debug_assert!(check, "Move already exists: {:?} -> {:?}\n", piece, to);
        self.attacked.inc(to);
    }

    /// Removes a move from the list of moves, returns whether the move was there to begin with
    pub fn remove(&mut self, piece: Index<Piece>, to: Square) -> bool {
        self.attacked.dec(to);
        self.moves.remove(&Move { piece, to })
    }

    /// Inserts an inactive move into the list of moves
    pub fn insert_inactive(&mut self, piece: Index<Piece>, to: Square) {
        // self.moves.insert(Move {
        //     piece,
        //     to,
        // });
        match self.callbacks.get_mut(&to) {
            Some(callbacks) => callbacks.push(piece),
            None => {
                self.callbacks.insert(to, vec![piece]);
            }
        }
    }

    /// Inserts a *good* move into the list of moves
    pub fn insert_good(&mut self, piece: Index<Piece>, to: Square) {
        self.moves.insert(Move {
            piece,
            to,
            // active: true,
        });
        self.attacked.inc(to);
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
            let piece = state.get_idx(idx).unwrap();
            let mut extra = String::new();
            let mut missing = String::new();
            let mut proper = String::new();

            for item in diff.items() {
                extra += " ";
                match item {
                    DiffItem::Extra(s) => extra += &s.to_string(),
                    DiffItem::Missing(s) => missing += &s.to_string(),
                    DiffItem::Proper(s) => proper += &s.to_string(),
                }
            }
            let add_line = |s: &mut String, prefix: &str| {
                if !s.is_empty() {
                    *s = format!("{prefix}{s}{}", '\n')
                }
            };
            add_line(&mut extra, "   + ");
            add_line(&mut missing, "   - ");
            add_line(&mut proper, &format!(" {} {}:", idx, piece.ch));

            match piece.team {
                Team::Black => write!(f, "{}", proper.green())?,
                Team::White => write!(f, "{}", proper.blue())?,
            }
            write!(f, "{}{}", missing.red(), extra.green())?;
        }

        Ok(())
    }
}
