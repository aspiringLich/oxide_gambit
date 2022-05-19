use super::{highlight::TargetSquare, mouse_event::cursor_square, MouseEvent, WindowInfo};
use crate::{
    chess_logic::*, interactive::highlight::toggle_target_squares, render::setup::vec_from_posz,
};
use bevy::{
    core::Time,
    math::Vec3,
    prelude::{
        Color, Commands, Component, Entity, EventReader, Local, Query, Res, ResMut, Transform, With,
    },
    sprite::Sprite,
    utils::Instant,
};

#[derive(Component)]
pub struct SelectedSquare(pub Piece);

pub fn drag_and_drop(
    mut commands: Commands,
    mut query_trans: Query<&mut Transform, With<SelectedSquare>>,
    mut query_sprite: Query<&mut Sprite, With<SelectedSquare>>,
    mut target_query: Query<Entity, With<TargetSquare>>,
    mut select_query: Query<&mut SelectedSquare>,
    window: Res<WindowInfo>,
    mut mouse_ev: EventReader<MouseEvent>,
    state: Res<ChessState>,
    time: Res<Time>,
) {
    let mut select_piece = select_query.single_mut();
    let mut p_trans = query_trans.single_mut();
    let mut p_sprite = query_sprite.single_mut();

    for event in mouse_ev.iter() {
        match event {
            // if you press the chessboard
            MouseEvent::PressChessboard(n) => {
                // if this is a valid square to move a piece
                if state.occupied(*n)
                    && state.team(*n) == state.turn
                    && select_piece.0.id.piece_id() == 0
                {
                    select_piece.0.id = PieceType(state.id(*n));
                // if we click somewhere, do a thing
                } else if select_piece.0.id.piece_id() != 0 {
                    p_trans.translation = Vec3::new(0.0, 0.0, -1.0);
                    select_piece.0.id.0 = 0;
                }
            }
        }
    }

    let map_range = |from: (f32, f32), to: (f32, f32), n: f32| {
        to.0 + (n - from.0) * (to.1 - to.0) / (from.1 - from.0)
    };
    const SELECT_COLOR: [[u8; 3]; 2] = [[255, 196, 12], [245, 199, 26]];

    // track the cursor square
    if select_piece.0.id.piece_id() != 0 {
        let on_square = cursor_square(&window);
        if on_square.is_none() {
            return;
        }
        let selected = on_square.unwrap();
        p_trans.translation = vec_from_posz(selected, 2.0);
        // change the color
        let secs = time.seconds_since_startup() as f32;
        let color: usize = ((selected.0 / 8 % 2 + selected.0 % 2) % 2).into();
        p_sprite.color = Color::rgba_u8(
            SELECT_COLOR[color][0],
            SELECT_COLOR[color][1],
            SELECT_COLOR[color][2],
            (map_range((-1.0, 1.0), (0.2, 0.4), (secs * 4.0).sin()) * 255.0) as u8,
        );
    }
}
