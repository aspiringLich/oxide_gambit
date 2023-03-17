use std::cell::RefCell;

use engine::{rules::Rules, state::state::State};

fn main() {
    let rules = RefCell::new(Rules::standard());
    let state = State::from_FEN(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 ",
        &rules,
    );
    dbg!(&state);
    println!("{}", state.unwrap())
}
