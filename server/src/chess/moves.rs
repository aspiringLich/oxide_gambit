use std::collections::VecDeque;

use super::square::Square;

/// a structure for holding the squares a piece can move to
pub struct Moves {
    inner: VecDeque<Square>,
}

// impl Moves {
//     pub fn generate()
// }
