#![feature(default_free_fn)]
#![feature(trait_alias)]
#![feature(decl_macro)]

use derive_more::{Deref, DerefMut};

mod chess;


struct e {
    f: i32,
}

#[proc_macros::builder_impl]
impl e {
    pub fn f(f: i32);
}