

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
    
    pub fn x(&self) -> i8 {
        match self {
            E => 1,
            NE => 1,
            N => 0,
            NW => -1,
            W => -1,
            SW => -1,
            S => 0,
            SE => 1
        }
    }
    
    pub fn y(&self) -> i8 {
        match self {
            E => 0,
            NE => 1,
            N => 1,
            NW => 1,
            W => 0,
            SW => -1,
            S => -1,
            SE => -1
        }
    }
}