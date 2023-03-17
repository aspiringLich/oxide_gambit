use std::hash::Hash;

/// A u8 that serves as an index into a list of `T`
#[derive(Debug)]
pub struct Index<T>(u8, std::marker::PhantomData<T>);

impl<T> Hash for Index<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl<T> PartialEq for Index<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for Index<T> {}

impl<T> Clone for Index<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Index<T> {}

impl<T> Default for Index<T> {
    fn default() -> Self {
        Self(0, std::marker::PhantomData)
    }
}

impl<T> Index<T> {
    pub fn get(self, arr: &[T]) -> &T {
        &arr[self.0 as usize]
    }

    pub fn new(idx: u8) -> Self {
        Self(idx, std::marker::PhantomData)
    }
}