use bevy::{
    hierarchy::{BuildChildren, DespawnRecursiveExt},
    math::Vec3,
    prelude::{
        Color, Commands, Component, Entity, EventReader, Query, Res, ResMut, Transform, With,
    },
    sprite::{Sprite, SpriteBundle},
    utils::Instant,
};

use crate::{
    interactive::SelectedSquare,
    render::setup::{vec_from_posz, SQ_SIZE},
};

use crate::chess_logic::{ChessState, Piece, PieceType, Pos};

use super::mouse_event::MouseEvent;

#[derive(Component)]
pub struct TargetSquare;

// initialize stuff for interactive
pub fn init_interactive(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: vec_from_posz(Pos(0), -1.0),
                scale: Vec3::new(SQ_SIZE, SQ_SIZE, 0.0),
                ..Default::default()
            },
            sprite: Sprite { color: Color::rgb_u8(245, 199, 26), ..Default::default() },
            ..Default::default()
        })
        .insert(SelectedSquare(Piece::new(Pos(0), PieceType(0))));
}

pub fn toggle_target_squares(
    state: Res<ChessState>,
    mut commands: Commands,
    select_query: Query<&mut SelectedSquare>,
    target_query: Query<Entity, With<TargetSquare>>,
    mut mouse_ev: EventReader<MouseEvent>,
) {
    let select_piece = select_query.single();
    if select_piece.0.piece_id() != 0 {
        // get all chess moves that start with the selected piece
        let mut to: Vec<Pos> = vec![];
        for chessmove in &state.moves {
            if chessmove.origin == select_piece.0.pos {
                to.push(chessmove.target);
            }
        }
        let target = commands.spawn().insert(TargetSquare).id();
        // spawn the target squares
        for pos in to {
            let entity = commands
                .spawn_bundle(SpriteBundle {
                    transform: Transform {
                        translation: vec_from_posz(pos, 2.5),
                        scale: Vec3::new(SQ_SIZE / 4.0, SQ_SIZE / 4.0, 0.0),
                        ..Default::default()
                    },
                    sprite: Sprite { color: Color::rgba_u8(0, 0, 0, 127), ..Default::default() },
                    ..Default::default()
                })
                .id();
            commands.entity(target).add_child(entity).insert(TargetSquare);
        }
    } else {
        commands.entity(target_query.single()).despawn_recursive();
    }
}