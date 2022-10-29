#![allow(warnings)] // comment to enjoy 8 hours of fixing warnings
#![feature(let_else)] // use for check_pins in pins.rs
#![feature(into_future)] // implemented on ChessMove
#![feature(test)] // for, you guessed it, benchmarking
#![feature(default_free_fn)] // i want my defaults

// //! UNCOMMENT THESE LATER

// mod benchmark;

// mod ai;
// mod chess;
// mod interactive;
// mod render;
// use bevy::prelude::*;
// use bevy_inspector_egui::WorldInspectorPlugin;
// use iyes_loopless::prelude::*;

// use chess::*;
// use interactive::*;
// use render::*;

// #[macro_use]
// extern crate lazy_static;

// enum PluginGroup {
//     Interactive, // interactive elements, drag and drop and whatnot
//     Debug,       // Debug your heart out
// }
// struct Holder(PluginGroup);
// impl Plugin for Holder {
//     fn build(&self, app: &mut App) {
//         use PluginGroup::*;
//         match self.0 {
//             Interactive => app
//                 .add_event::<MouseEvent>()
//                 .insert_resource(WindowInfo::empty())
//                 .add_startup_system(init_interactive)
//                 .add_system(send_mouse_events.before("window"))
//                 .add_system(update_window_info.label("window"))
//                 // spawn highlight and target squares
//                 .add_system_set(
//                     SystemSet::new()
//                         .label("spawn squares")
//                         .after("window")
//                         .with_system(toggle_select_square.run_on_event::<MouseEvent>().label("select"))
//                         .with_system(
//                             toggle_target_squares
//                             .run_on_event::<MouseEvent>()
//                             .after("select")
//                         )
//                     )
//                 .add_system(
//                     update_select_square
//                         .run_unless_resource_equals(Piece::default())
//                         .after("spawn squares")
//                 )
//                 .add_system(
//                     attempt_move_piece
//                         .run_on_event::<MouseEvent>()
//                         .run_unless_resource_equals(Piece::default())
//                         .label("move")
//                         .after("window")
//                         .before("select")
//                 )
//                 .add_system(
//                     excecute_calc_task
//                     .after("move")),
//             Debug => app
//                 .add_plugin(WorldInspectorPlugin::new())
//                 // .add_startup_system(init_threat_squares)
//                 // .add_system(update_threat_squares.after("move")),
//         };
//     }
// }

// enum StartingPos {
//     Standard = 0,
//     Castling,
//     EnPassant,
//     Promotion,
// }
// use StartingPos::*;

// use crate::ai::ai_sys::excecute_calc_task;

// const starting_pos: [&str; 4] = [
//     "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 ",
//     "r3k2r/3q4/8/8/8/8/3Q4/R3K2R w KQkq - 0",
//     "rnbqkbnr/pppppppp/8/4P3/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 ",
//     "8/PPPPPPPP/K7/8/8/k7/pppppppp/8 w - - 0 ",
// ];

// fn main() {
//     use PluginGroup::*;

//     App::new()
//         .insert_resource(WindowDescriptor {
//             // position: Some(Vec2::new(0.0, 0.0)),
//             title: "Oxide Gambit".to_string(),
//             // width: 1920.0,
//             // height: 1080.0,
//             ..Default::default()
//         })
//         .add_plugins(DefaultPlugins)
//         .add_startup_system(setup::setup)
//         .add_plugin(Holder(Interactive))
//         .add_plugin(Holder(Debug))
//         .insert_resource(ChessState::from_FEN(starting_pos[Standard as usize]))
//         .insert_resource(Piece::default())
//         .run();
// }

fn main() {}
