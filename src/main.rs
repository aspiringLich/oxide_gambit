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

use crate::chess_logic::{Piece, PieceType, PieceVariant, Position};

enum PluginGroup {
    Interactive, // interactive elements, drag and drop and whatnot
}
struct Holder(PluginGroup);
impl Plugin for Holder {
    fn build(&self, app: &mut App) {
        use PieceVariant::*;
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
                        .after("window")
                        .before("select")
                ),
        };
    }
}

fn main() {
    let starting_pos: String =
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();
    let rook_test: String = "8/8/8/1B6/2R5/8/1P6/8 w KQkq - 0 1".to_string();
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
        .insert_resource(Piece::default())
        .run();
}
