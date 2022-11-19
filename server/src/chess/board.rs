use super::{pieces::Piece, square::Square};
use std::ops::{Deref, DerefMut, Index, IndexMut};
use yauc::prelude::*;

#[derive(Derivative, Debug)]
#[derivative(Default)]
pub struct Board {
    #[derivative(Default(value = "[default(); 64]"))]
    inner: [Piece; 64],
}

impl Deref for Board {
    type Target = [Piece; 64];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Board {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Index<Square> for Board {
    type Output = Piece;

    fn index(&self, index: Square) -> &Self::Output {
        &self.inner[*index as usize]
    }
}

impl IndexMut<Square> for Board {
    fn index_mut(&mut self, index: Square) -> &mut Self::Output {
        &mut self.inner[*index as usize]
    }
}
