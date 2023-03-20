
use engine::{
    chess,
    move_gen,
    rules,
    state,
};
use bevy::prelude::default;
mod assets;

fn main() {
    engine::init();

    let rules = std::cell::RefCell::new(rules::Rules::standard());
    let state = state::State::from_FEN(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 ",
        &rules,
    )
    .unwrap();

    dbg!(&state);
    println!("{}", state);
    
    
}
