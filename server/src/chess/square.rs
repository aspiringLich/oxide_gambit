use std::ops::{Deref, DerefMut};

#[derive(Default, Debug)]
pub struct Square {
    square: u8,
}

impl Square {
    pub fn new(square: u8) -> Self {
        Square { square }
    }
}

impl Deref for Square {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.square
    }
}

impl DerefMut for Square {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.square
    }
}
