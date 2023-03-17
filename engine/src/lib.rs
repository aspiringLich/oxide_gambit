#![feature(default_free_fn)]
#![feature(trait_alias)]
#![feature(decl_macro)]
#![feature(let_chains)]
#![feature(iter_array_chunks)]

use derive_more::{Deref, DerefMut};

pub mod chess;
pub mod move_gen;
pub mod rules;
pub mod state;
