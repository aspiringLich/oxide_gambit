use std::{fmt::Write, hash::Hash};

use crate::misc::u8_to_char;

/// A u8 that serves as an index into a list of `T`
#[derive(Debug)]
pub struct Index<T>(u8, std::marker::PhantomData<T>);

impl<T> std::fmt::Display for Index<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(u8_to_char(self.0))
    }
}

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

impl<T> PartialOrd for Index<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Index<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

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

    pub fn get_mut(self, arr: &mut [T]) -> &mut T {
        &mut arr[self.0 as usize]
    }

    pub fn new(idx: u8) -> Self {
        Self(idx, std::marker::PhantomData)
    }
}
