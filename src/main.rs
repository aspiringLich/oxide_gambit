mod ai;
mod chess_logic;
mod interactive;
mod render;
use bevy::prelude::*;
use iyes_loopless::prelude::*;

use ai::*;
use chess_logic::ChessState;
use interactive::*;
use render::*;

use crate::chess_logic::{Piece, PieceType, Pos};

enum PluginGroup {
    Interactive, // interactive elements, drag and drop and whatnot
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
                .add_system(send_mouse_events.before(update_window_info))
                .add_system(update_window_info)
                .add_system(
                    toggle_select_square.run_on_event::<MouseEvent>().after(update_window_info),
                )
                .add_system(
                    update_select_square
                        .run_unless_resource_equals(Piece::new(Pos(0), PieceType(0)))
                        .after(toggle_select_square),
                ),
        };
    }
}

fn main() {
    let starting_pos: String =
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();
    let rook_test: String = "8/8/8/3R4/8/8/8/8 w KQkq - 0 1".to_string();
    // let state: State = State::from_FEN(&starting_pos);
    // dbg!(state);
    use PluginGroup::*;

    App::new()
        .insert_resource(WindowDescriptor {
            position: Some(Vec2::new(0.0, 0.0)),
            title: "Oxide Gambit".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup::setup)
        .add_plugin(Holder(Interactive))
        .insert_resource(ChessState::from_FEN(&rook_test))
        .insert_resource(Piece::new(Pos(0), PieceType(0)))
        .run();
}
