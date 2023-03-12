use super::square::Square;


pub struct Move {
    pub from: Square,
    pub to: Square,
}

pub struct Moves {
    moves: Vec<Move>
}

impl Moves {
    pub fn new() -> Moves {
        Moves { moves: Vec::new() }
    }

    pub fn add(&mut self, from: Square, to: Square) {
        self.moves.push(Move { from, to });
    }
}