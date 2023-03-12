#![feature(default_free_fn)]
#![feature(trait_alias)]
#![feature(decl_macro)]

use derive_more::{Deref, DerefMut};

mod chess;



#[test]
fn test_e() {
    struct e {
        f: i32,
    }
    
    #[proc_macros::builder_impl]
    impl e {
        /// gaming
        pub fn f(f: i32) => f + 1;
    }
    
    let E = e { f: 0 };
    assert_eq!(E.f(1).f, 2);
}