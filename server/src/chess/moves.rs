use std::{
    collections::VecDeque,
    ops::{Deref, DerefMut},
};

use super::square::Square;

/// a structure for holding the squares a piece can move to
pub struct Moves {
    inner: VecDeque<Square>,
}

impl Deref for Moves {
    type Target = VecDeque<Square>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Moves {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
