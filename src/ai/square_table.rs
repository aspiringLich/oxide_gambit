use super::*;
use crate::*;

// piece square tables stolen from chessprogramming.org
const PAWN_TABLE_RAW: [i8; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 50, 50, 50, 50, 50, 50, 50, 50, 10, 10, 20, 30, 30, 20, 10, 10, 5, 5,
    10, 25, 25, 10, 5, 5, 0, 0, 0, 20, 20, 0, 0, 0, 5, -5, -10, 0, 0, -10, -5, 5, 5, 10, 10, -20,
    -20, 10, 10, 5, 0, 0, 0, 0, 0, 0, 0, 0,
];
const KNIGHT_TABLE_RAW: [i8; 64] = [
    -50, -40, -30, -30, -30, -30, -40, -50, -40, -20, 0, 0, 0, 0, -20, -40, -30, 0, 10, 15, 15, 10,
    0, -30, -30, 5, 15, 20, 20, 15, 5, -30, -30, 0, 15, 20, 20, 15, 0, -30, -30, 5, 10, 15, 15, 10,
    5, -30, -40, -20, 0, 5, 5, 0, -20, -40, -50, -40, -30, -30, -30, -30, -40, -50,
];
const BISHOP_TABLE_RAW: [i8; 64] = [
    -20, -10, -10, -10, -10, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 10, 10, 5, 0,
    -10, -10, 5, 5, 10, 10, 5, 5, -10, -10, 0, 10, 10, 10, 10, 0, -10, -10, 10, 10, 10, 10, 10, 10,
    -10, -10, 5, 0, 0, 0, 0, 5, -10, -20, -10, -10, -10, -10, -10, -10, -20,
];
const ROOK_TABLE_RAW: [i8; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 5, 10, 10, 10, 10, 10, 10, 5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0,
    0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, 0, 0,
    0, 5, 5, 0, 0, 0,
];
const QUEEN_TABLE_RAW: [i8; 64] = [
    -20, -10, -10, -5, -5, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 5, 5, 5, 0, -10,
    -5, 0, 5, 5, 5, 5, 0, -5, 0, 0, 5, 5, 5, 5, 0, -5, -10, 5, 5, 5, 5, 5, 0, -10, -10, 0, 5, 0, 0,
    0, 0, -10, -20, -10, -10, -5, -5, -10, -10, -20,
];
const KING_MID_TABLE_RAW: [i8; 64] = [
    -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40,
    -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -20, -30, -30, -40, -40, -30,
    -30, -20, -10, -20, -20, -20, -20, -20, -20, -10, 20, 20, 0, 0, 0, 0, 20, 20, 20, 30, 10, 0, 0,
    10, 30, 20,
];
const KING_END_TABLE_RAW: [i8; 64] = [
    -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40,
    -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -20, -30, -30, -40, -40, -30,
    -30, -20, -10, -20, -20, -20, -20, -20, -20, -10, 20, 20, 0, 0, 0, 0, 20, 20, 20, 30, 10, 0, 0,
    10, 30, 20,
];

fn generate_from_table(table: [i8; 64]) -> [f32; 64] {
    let mut out: [f32; 64] = [0.0; 64];
    for i in 0..64 {
        out[i] = table[i] as f32 * SQUARE_MULTIPLIER;
    }
    return out;
}

lazy_static! {
    static ref PAWN_TABLE: [f32; 64] = generate_from_table(PAWN_TABLE_RAW);
    static ref KNIGHT_TABLE: [f32; 64] = generate_from_table(KNIGHT_TABLE_RAW);
    static ref BISHOP_TABLE: [f32; 64] = generate_from_table(BISHOP_TABLE_RAW);
    static ref ROOK_TABLE: [f32; 64] = generate_from_table(ROOK_TABLE_RAW);
    static ref QUEEN_TABLE: [f32; 64] = generate_from_table(QUEEN_TABLE_RAW);
    static ref KING_MID_TABLE: [f32; 64] = generate_from_table(KING_MID_TABLE_RAW);
    static ref KING_END_TABLE: [f32; 64] = generate_from_table(KING_END_TABLE_RAW);
}

/// initialize all the piece tables
pub fn initialize_piece_tables() {
    use lazy_static::initialize;
    initialize(&PAWN_TABLE);
    initialize(&KNIGHT_TABLE);
    initialize(&BISHOP_TABLE);
    initialize(&ROOK_TABLE);
    initialize(&QUEEN_TABLE);
    initialize(&KING_MID_TABLE);
    initialize(&KING_END_TABLE);
}

impl Piece {
    pub fn get_square_value(&self, endgame: bool) -> f32 {
        use PieceVariant::*;

        let pos = if self.team() { Position((7 - self.y()) * 8 + self.x()) } else { self.position };
        let out = match self.variant() {
            Pawn => PAWN_TABLE[pos.int()],
            Knight => KNIGHT_TABLE[pos.int()],
            Bishop => BISHOP_TABLE[pos.int()],
            Rook => ROOK_TABLE[pos.int()],
            Queen => QUEEN_TABLE[pos.int()],
            King if !endgame => KING_MID_TABLE[pos.int()],
            King => KING_END_TABLE[pos.int()],
            None => 0.0,
        };
        return if self.team() { out } else { -out };
    }
}
