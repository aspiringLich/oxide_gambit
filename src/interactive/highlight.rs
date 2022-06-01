use bevy::{
    hierarchy::{BuildChildren, DespawnRecursiveExt, Parent},
    math::Vec3,
    prelude::{
        Color, Commands, Component, Entity, EventReader, Query, Res, ResMut, Transform, Visibility,
        With,
    },
    sprite::{Sprite, SpriteBundle},
    utils::Instant,
};

use crate::{
    chess_logic::PieceVariant,
    interactive::SelectedSquare,
    render::setup::{vec_from_posz, SQ_SIZE},
};

use crate::chess_logic::{ChessState, Piece, PieceType, Position};

use super::mouse_event::MouseEvent;

#[derive(Component)]
pub struct TargetSquare;

// initialize stuff for interactive
pub fn init_interactive(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: vec_from_posz(Default::default(), 2.0),
                scale: Vec3::new(SQ_SIZE, SQ_SIZE, 0.0),
                ..Default::default()
            },
            sprite: Sprite { color: Color::rgb_u8(245, 199, 26), ..Default::default() },
            visibility: Visibility { is_visible: false },
            ..Default::default()
        })
        .insert(SelectedSquare());
}

pub fn toggle_target_squares(
    state: Res<ChessState>,
    mut commands: Commands,
    piece: Res<Piece>,
    mut target_query: Query<Entity, With<TargetSquare>>,
) {
    use PieceVariant::*;

    // run after you set piece variant
    if piece.variant() != None {
        // get all chess moves that start with the selected piece
        let mut to: Vec<Position> = vec![];
        for chessmove in &state.moves {
            if chessmove.origin == piece.position {
                to.push(chessmove.target);
            }
        }

        // spawn the target squares
        for pos in to {
            commands
                .spawn_bundle(SpriteBundle {
                    transform: Transform {
                        translation: vec_from_posz(pos, 2.5),
                        scale: Vec3::new(SQ_SIZE / 4.0, SQ_SIZE / 4.0, 0.0),
                        ..Default::default()
                    },
                    sprite: Sprite { color: Color::rgba_u8(0, 0, 0, 127), ..Default::default() },
                    ..Default::default()
                })
                .insert(TargetSquare);
        }
    } else {
        for entity in target_query.iter_mut() {
            commands.entity(entity).despawn();
        }
    }
}
