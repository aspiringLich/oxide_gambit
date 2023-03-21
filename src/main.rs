#![feature(const_fn_floating_point_arithmetic)]
#![feature(decl_macro)]
#![feature(let_chains)]

mod assets;
mod board;
mod drag;
mod misc;
mod theme;

use bevy::prelude::*;
use engine::{rules, state};
use misc::EntityNamer;

fn main() {
    engine::init();

    // let rules = std::cell::RefCell::new(rules::Rules::standard());
    // let state =
    // .unwrap();
    // println!("{}", state);

    let mut app = App::new();

    app.register_type::<TextureAtlasSprite>()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..default()
                }),
        )
        .add_plugin(bevy_inspector_egui::quick::WorldInspectorPlugin::new())
        .add_event::<drag::ClickEvent>()
        .add_event::<drag::DragEvent>()
        .add_startup_systems((assets::init, theme::init, board::init, drag::init))
        .add_system(board::spawn_board)
        .add_systems((
            drag::update_mouse_pos,
            drag::update_hovered_tile.after(drag::update_mouse_pos),
            drag::click_event_sender.after(drag::update_hovered_tile),
            drag::drag_event_sender.after(drag::update_hovered_tile),
            
        ));

    app.run()
}
