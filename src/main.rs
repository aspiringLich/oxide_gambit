#![feature(const_fn_floating_point_arithmetic)]
#![feature(decl_macro)]

use bevy::prelude::*;
use engine::{chess, move_gen, rules, state};

mod assets;
mod board;
mod theme;

fn main() {
    engine::init();

    // let rules = std::cell::RefCell::new(rules::Rules::standard());
    // let state = 
    // .unwrap();
    // println!("{}", state);

    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_startup_systems((assets::init, theme::init, board::init))
        .add_system(board::spawn_board);

    app.run()
}
