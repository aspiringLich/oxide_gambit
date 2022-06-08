use bevy::{
    core::Name,
    hierarchy::{BuildChildren, DespawnRecursiveExt, Parent},
    math::Vec3,
    prelude::*,
    sprite::{Sprite, SpriteBundle},
    utils::Instant,
};

use crate::{
    chess_logic::PieceVariant,
    interactive::{SelectedSquare, TargetSquare},
    render::setup::{vec_from_posz, SQ_SIZE},
};

use crate::chess_logic::{ChessState, Piece, PieceType, Position};

use super::mouse_event::MouseEvent;

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
        .insert(SelectedSquare())
        .insert(Name::new("Selected Square"));

    commands
        .spawn_bundle(SpriteBundle { ..default() })
        .insert(TargetSquare)
        .insert(Name::new("Target Square Parent"));
}

pub fn toggle_target_squares(
    state: Res<ChessState>,
    mut commands: Commands,
    piece: Res<Piece>,
    mut target_query: Query<Entity, With<TargetSquare>>,
) {
    use PieceVariant::*;

    let parent = target_query.single_mut();

    // run after you set piece variant
    if piece.variant() != None {
        // spawn the target squares
        for pos in state.moves.iter().filter(|x| x.origin == piece.position).map(|x| x.target) {
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
                .insert(Parent(parent))
                .insert(Name::new("Target Marker"));
        }
    } else {
        commands.entity(parent).despawn_descendants()
    }
}
