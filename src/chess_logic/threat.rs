use bevy::prelude::*;

use crate::{
    chess_logic::{
        position::{coord_to_index, index_to_coord, is_45},
        PieceVariant,
    },
    render::{vec_from_coord, vec_from_posz, SQ_SIZE},
};

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

pub fn possible_threat(variant: PieceVariant, dir: usize) -> bool {
    use PieceVariant::*;
    variant == Queen || variant == [Bishop, Rook][dir % 2]
}

const DEBUG: bool = false;

impl ChessState {
    /// remove threats from a range of positions
    /// assumes that the positions are diagonal / orthogonal from each other
    /// includes new_pos, does not include pos
    fn rem_from_pos_range(&mut self, mut pos: Position, new_pos: Position, (x, y): (i8, i8)) {
        let old_pos = pos;
        // we actually got blocked off by the piece oh noess
        while pos != new_pos {
            pos.modify(x + y * 8);
            if DEBUG {
                eprintln!("rem pos range: {}", pos.0);
            }
            self.rem_threat(Piece::new(self.at(old_pos), old_pos), pos);
        }
    }

    /// same as remove but add
    /// wow
    fn add_from_pos_range(&mut self, mut pos: Position, new_pos: Position, (x, y): (i8, i8)) {
        let old_pos = pos;
        // we actually got blocked off by the piece oh noess
        while pos != new_pos {
            pos.modify(x + y * 8);
            if DEBUG {
                eprintln!("add pos range: {}", pos.0);
            }
            self.add_threat(Piece::new(self.at(old_pos), old_pos), pos);
        }
    }

    /// do before updating state, doesnt update the squares of a captured piece
    pub fn update_threat(&mut self, piece: Piece, new_pos: Position) {
        use PieceVariant::*;

        self.rem_threat_piece(piece);
        self.add_threat_piece(Piece::new(piece.variant, new_pos));

        let pieces: &[Vec<Piece>; 2] = unsafe { std::mem::transmute(&self.pieces) };

        let (x, y) = piece.rel_from(new_pos);
        let move_dir = if piece.variant() == Knight { 4 } else { coord_to_index(x, y) };

        // you moved in a way that blocks yourself off
        if piece.variant() == Queen || piece.variant() == [Bishop, Rook][move_dir % 2] {
            self.add_threat_dir(piece, index_to_coord(8 - move_dir));
        }

        // closest piece on each diagonal / orthagonal
        let mut unblock_piece: [(Piece, i8); 9] = [(default(), i8::MAX); 9];
        let mut block_piece: [(Piece, i8); 9] = [(default(), i8::MAX); 9];

        let check_piece = |array: &mut [(Piece, i8); 9], (x, y): (i8, i8), piece: Piece| {
            if is_45(x, y) {
                let max = if x == 0 { y.abs() } else { x.abs() };
                let index = coord_to_index(x, y);
                if max < array[index].1 {
                    array[index] = (piece, max);
                }
            }
        };

        // go through the pieces and find the closest ones
        for &new_piece in pieces[0].iter().chain(pieces[1].iter()) {
            let (x, y) = piece.rel_from(new_piece.position);
            check_piece(&mut unblock_piece, (x, y), new_piece);

            let (x, y) = new_pos.rel_from(new_piece.position);
            // if unblock_piece[coord_to_index(x, y)].0 != new_piece {
            check_piece(&mut block_piece, (x, y), new_piece);
            // }
        }
        // reset middle
        unblock_piece[4] = default();
        block_piece[4] = default();

        // remove our piece
        block_piece[8 - move_dir] = default();

        eprintln!("Unblocked: ");
        for item in unblock_piece {
            dbg!(item.0);
        }
        eprintln!("Blocked: ");
        for item in block_piece {
            dbg!(item.0);
        }

        // TODO (?): maybe you could use the distance to the piece to calculate threatened squares more efficiently?

        // for each piece we could unblock, check if its valid,
        // if it satisfies criteria, remove all its threatenned squares in that direction and regenerate
        for (i, (check_piece, _)) in unblock_piece.iter().enumerate() {
            let variant = check_piece.variant();
            if possible_threat(variant, i) {
                // eprint!("gaming: ");

                if check_piece.position == new_pos {
                    // remove the threatened piece's targetted squares
                    self.rem_threat_piece(Piece::new(self.at(new_pos), new_pos));
                }
                // if the piece moved on the same axis, reset threatenned squares and check until the piece
                else if i == move_dir || i == 8 - move_dir {
                    if DEBUG {
                        eprint!("same axis: {} => {}: ", check_piece.position.0, new_pos.0);
                    }
                    let (x, y) = index_to_coord(8 - i);
                    self.rem_from_pos_range(check_piece.position, piece.position, (x, y));
                    self.add_from_pos_range(check_piece.position, new_pos, (x, y));
                }
                // just extend out the squares
                else {
                    if DEBUG {
                        eprint!(
                            "extend: {} & {} {}+: ",
                            check_piece.position.0,
                            8 - i,
                            piece.position.0
                        );
                    }
                    self.add_threat_dir(
                        Piece::new(check_piece.variant, piece.position),
                        index_to_coord(8 - i),
                    )
                }
            }
        }

        // for each piece that we could block, same dealio check if its valid
        // if it satisfies yadda yadda remove threatenned squares in that direction
        for (i, (check_piece, _)) in block_piece.iter().enumerate() {
            let variant = check_piece.variant();
            if i != move_dir && i != 8 - move_dir && possible_threat(check_piece.variant(), i) {
                if DEBUG {
                    eprint!("block: {} & {}", check_piece.position.0, 8 - i);
                }
                dbg!(check_piece);

                let piece = Piece::new(check_piece.variant, new_pos);
                let dir = index_to_coord(8 - i);
                if let Some(pos) = piece.try_to(dir) {
                    // if the next square will be a zero return
                    // we dont need to remove anything
                    if self.threat_at(pos, self.turn) == 0 {
                        continue;
                    }
                    self.rem_threat_dir(piece, dir);
                }
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
    fn add_threat_dir(&mut self, piece: Piece, (x, y): (i8, i8)) {
        let mut itr = 1;
        while let Some(pos) = piece.try_to((x * itr, y * itr)) {
            if DEBUG {
                eprintln!("add dir: {}", pos.0);
            }
            self.add_threat(piece, pos);
            if self.occupied(pos) {
                return;
            }
            itr += 1;
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
    fn rem_threat_dir(&mut self, piece: Piece, (x, y): (i8, i8)) {
        let mut itr = 1;

        while let Some(pos) = piece.position.try_to((x * itr, y * itr)) {
            eprintln!("rem dir: {}", pos.0);
            self.rem_threat(piece, pos);
            if self.occupied(pos) {
                return;
            }
            itr += 1;
        }
    }

    #[inline]
    fn rem_threat(&mut self, piece: Piece, pos: Position) {
        // dbg!(self.threatened[piece.team() as usize].squares[pos.int()]);
        self.threatened[piece.team() as usize].squares[pos.int()] -= 1;
    }
}

/// stores the ids for every entity in the threat square thingie
#[derive(Component)]
pub struct ThreatEntity(u8);

pub fn init_threat_squares(mut commands: Commands) {
    for n in 0..64u8 {
        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: vec_from_posz(Position(n), 5.0),
                    scale: Vec3::new(SQ_SIZE, SQ_SIZE, 0.0),
                    ..Default::default()
                },
                sprite: Sprite { color: Color::rgba_u8(255, 0, 0, 128), ..Default::default() },
                visibility: Visibility { is_visible: false },
                ..Default::default()
            })
            .insert(ThreatEntity(n));
    }
}

pub fn update_threat_squares(
    state: Res<ChessState>,
    mut query: Query<(&mut Visibility, &ThreatEntity)>,
) {
    for (mut visibility, &ThreatEntity(square)) in query.iter_mut() {
        visibility.is_visible = state.threatened[1].squares[square as usize] > 0;
        //    || state.threatened[0].squares[square as usize] > 0;
    }
}
