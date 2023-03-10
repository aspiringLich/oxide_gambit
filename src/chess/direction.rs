

pub enum Direction {
    E,
    NE,
    N,
    NW,
    W,
    SW,
    S,
    SE
}

use Direction::*;
impl Direction {
    const ORTHOGONAL: [Direction; 4] = [E, N, W, S];
    const DIAGONAL: [Direction; 4] = [NE, NW, SW, SE];
    const ALL: [Direction; 8] = [E, NE, N, NW, W, SW, S, SE];
}