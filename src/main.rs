mod ai;
mod render;
use bevy::prelude::*;

use ai::State;
use render::setup;

fn main() {
    let starting_pos: String =
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();
    let rook_test: String = "8/8/8/3R4/8/8/8/8 w KQkq - 0 1".to_string();
    // let state: State = State::from_FEN(&starting_pos);
    // dbg!(state);

    App::new()
        .insert_resource(WindowDescriptor {
            position: Some(Vec2::new(0.0, 0.0)),
            title: "Oxide Gambit".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup::setup)
        .insert_resource(State::from_FEN(&rook_test))
        .run();
}
