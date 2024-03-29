#![feature(default_free_fn)]
#![feature(trait_alias)]
#![feature(decl_macro)]
#![feature(let_chains)]
#![feature(iter_array_chunks)]
#![feature(adt_const_params)]
#![feature(const_trait_impl)]
#![feature(derive_const)]
#![feature(const_discriminant)]

use derive_more::{Deref, DerefMut};

pub mod chess;
pub mod misc;
pub mod move_gen;
pub mod rules;
pub mod state;

/// Initialize some static muts (i know i know)
pub fn init() {
    state::board_state::init();
    move_gen::normal::init();
}
