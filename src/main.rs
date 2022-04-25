pub use bevy::prelude::*;
mod render;
mod setup;
mod state;
mod theme;
mod move_gen;
mod piece;
use state::State;

fn main() {
    let starting_pos: String =
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();
    let state: State = State::from_FEN(&starting_pos);
    dbg!(state);

    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup::setup)
        .insert_resource(State::from_FEN(&starting_pos))
        .run();
}
