use std::cell::RefCell;

use engine::{rules::Rules, state::{state::State, board_state}};

fn main() {
    board_state::init();
    
    let rules = RefCell::new(Rules::standard());
    let state = State::from_FEN(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 ",
        &rules,
    ).unwrap();
    dbg!(&state);
    println!("{}", state);
    // dbg!(state.board_state);
}
