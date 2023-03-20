#![feature(const_fn_floating_point_arithmetic)]
#![feature(decl_macro)]

mod assets;
mod board;
mod misc;
mod theme;

use bevy::prelude::*;
use engine::{chess, move_gen, rules, state};
use misc::EntityNamer;

fn main() {
    engine::init();

    // let rules = std::cell::RefCell::new(rules::Rules::standard());
    // let state =
    // .unwrap();
    // println!("{}", state);

    let mut app = App::new();

    app.register_type::<TextureAtlasSprite>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()).set(AssetPlugin { watch_for_changes: true, ..default()}))
        .add_plugin(bevy_inspector_egui::quick::WorldInspectorPlugin::new())
        .add_startup_systems((assets::init, theme::init, board::init))
        .add_system(board::spawn_board);

    app.run()
}
