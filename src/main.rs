#![allow(warnings)] // comment to enjoy 8 hours of fixing warnings
#![feature(let_else)] // use for check_pins in pins.rs

mod ai;
mod chess_logic;
mod interactive;
mod render;
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use iyes_loopless::prelude::*;

use chess_logic::*;
use interactive::*;
use render::*;

#[macro_use]
extern crate lazy_static;

enum PluginGroup {
    Interactive, // interactive elements, drag and drop and whatnot
    Debug,       // Debug your heart out
}
struct Holder(PluginGroup);
impl Plugin for Holder {
    fn build(&self, app: &mut App) {
        use PluginGroup::*;
        match self.0 {
            Interactive => app
                .add_event::<MouseEvent>()
                .insert_resource(WindowInfo::empty())
                .add_startup_system(init_interactive)
                .add_system(send_mouse_events.before("window"))
                .add_system(update_window_info.label("window"))
                // spawn highlight and target squares
                .add_system_set(
                    SystemSet::new()
                        .label("spawn squares")
                        .after("window")
                        .with_system(toggle_select_square.run_on_event::<MouseEvent>().label("select"))
                        .with_system(
                            toggle_target_squares
                            .run_on_event::<MouseEvent>()
                            .after("select")
                        )
                    )
                .add_system(
                    update_select_square
                        .run_unless_resource_equals(Piece::default())
                        .after("spawn squares")
                )
                .add_system(
                    update_move
                        .run_on_event::<MouseEvent>()
                        .run_unless_resource_equals(Piece::default())
                        .label("move")
                        .after("window")
                        .before("select")
                ),
            Debug => app
                .add_plugin(WorldInspectorPlugin::new())
                // .add_startup_system(init_threat_squares)
                // .add_system(update_threat_squares.after("move")),
        };
    }
}

enum StartingPos {
    Standard = 0,
    Castling,
    EnPassant,
}
use StartingPos::*;

const starting_pos: [&str; 3] = [
    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 ",
    "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0",
    "rnbqkbnr/pppppppp/8/4P3/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 ",
];

fn main() {
    // let starting_pos: String =
    //     "rnbqkbnr/pppppppp/8/1N6/8/8/PPPPPPPP/R1BQKBNR w KQkq - 0 1".to_string();
    // let starting_pos: String = "8/2b5/8/1B6/2R5/8/1P6/8 w KQkq - 0 1".to_string();
    // let state: State = State::from_FEN(&starting_pos);
    // dbg!(state);
    use PluginGroup::*;

    App::new()
        .insert_resource(WindowDescriptor {
            // position: Some(Vec2::new(0.0, 0.0)),
            title: "Oxide Gambit".to_string(),
            // width: 1920.0,
            // height: 1080.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup::setup)
        .add_plugin(Holder(Interactive))
        .add_plugin(Holder(Debug))
        .insert_resource(ChessState::from_FEN(starting_pos[EnPassant as usize]))
        .insert_resource(Piece::default())
        .run();
}
