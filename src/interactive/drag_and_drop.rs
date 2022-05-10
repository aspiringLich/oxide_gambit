use super::{mouse_event::cursor_square, MouseEvent, WindowInfo};
use crate::{chess_logic::*, render::setup::vec_from_posz};
use bevy::{
    math::Vec3,
    prelude::{
        Commands, Component, Entity, EventReader, Local, Query, Res, ResMut, Transform, With,
    },
};

#[derive(Component)]
pub struct HeldPiece(pub Piece);

pub fn drag_and_drop(
    mut commands: Commands,
    mut query: Query<&mut Transform, With<HeldPiece>>,
    mut piece_query: Query<&mut HeldPiece>,
    window: Res<WindowInfo>,
    mut mouse_ev: EventReader<MouseEvent>,
    state: Res<ChessState>,
) {
    let mut held_piece = piece_query.single_mut();
    let mut piece = query.single_mut();

    for event in mouse_ev.iter() {
        match event {
            MouseEvent::PressChessboard(n) => {
                if state.occupied(*n)
                    && state.team(*n) == state.turn
                    && held_piece.0.id.piece_id() == 0
                {
                    let on_square = cursor_square(&window);
                    if on_square.is_none() {
                        continue;
                    }
                    piece.translation = vec_from_posz(on_square.unwrap(), 2.0);

                    held_piece.0.id = PieceType(state.id(*n));
                } else if held_piece.0.id.piece_id() != 0 {
                    piece.translation = Vec3::new(0.0, 0.0, -1.0);
                    held_piece.0.id.0 = 0;
                }
            }
        }
    }

    // keep tracking the piece
    if held_piece.0.id.piece_id() != 0 {
        let on_square = cursor_square(&window);
        if on_square.is_none() {
            return;
        }
        piece.translation = vec_from_posz(on_square.unwrap(), 2.0);
    }
}
