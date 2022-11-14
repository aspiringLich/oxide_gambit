#![feature(const_trait_impl)]
#![feature(test)]
#![feature(slice_index_methods)]
#![feature(iter_array_chunks)]
extern crate test;

// mod ai;
mod chess;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
