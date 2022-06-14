use super::{mouse_event::cursor_square, MouseEvent, SelectedSquare, WindowInfo};
use crate::{
    chess_logic::*,
    interactive::highlight::toggle_target_squares,
    render::{setup::vec_from_posz, DrawnPiece},
};
use bevy::{core::Time, prelude::*, sprite::Sprite};
use bevy_prototype_debug_lines::DebugLines;

pub fn toggle_select_square(
    mut visib_quert: Query<&mut Visibility, With<SelectedSquare>>,
    mut piece: ResMut<Piece>,
    mut mouse_ev: EventReader<MouseEvent>,
    state: Res<ChessState>,
) {
    use MouseEvent::*;
    use PieceVariant::*;

    let piece = piece.as_mut();

    let event = mouse_ev.iter().next();
    if let Some(PressChessboard(pos)) = event {
        // if this is a valid square to move a piece
        let mut visibility = visib_quert.single_mut();
        if state.occupied(*pos) && state.team(*pos) == state.turn && piece.variant() == None {
            visibility.is_visible = true;
            *piece = Piece::new(state.at(*pos), *pos);
        // if we click somewhere, do a thing
        } else if piece.variant() != None {
            visibility.is_visible = false;
            *piece = default();
        }
    }
}

pub fn update_select_square(
    mut comp_query: Query<(&mut Sprite, &mut Transform), With<SelectedSquare>>,
    window: Res<WindowInfo>,
    time: Res<Time>,
) {
    // extract the components from the entity
    let (mut sprite, mut transform) = comp_query.single_mut();

    // make sure we actually need to do this in the first place
    // get the cursor position
    let pos = match window.chessboard_pos() {
        Some(n) => n,
        None => return,
    };

    // some constant stuffs
    fn map_range(from: (f32, f32), to: (f32, f32), n: f32) -> f32 {
        to.0 + (n - from.0) * (to.1 - to.0) / (from.1 - from.0)
    };
    const SELECT_COLOR: [[u8; 3]; 2] = [[255, 196, 12], [245, 199, 26]];

    // track the cursor square
    transform.translation = vec_from_posz(pos, 2.0);

    // change the color
    let secs = time.seconds_since_startup() as f32;
    let color: usize = ((pos.0 / 8 % 2 + pos.0 % 2) % 2).into();
    sprite.color = Color::rgba_u8(
        SELECT_COLOR[color][0],
        SELECT_COLOR[color][1],
        SELECT_COLOR[color][2],
        (map_range((-1.0, 1.0), (0.2, 0.4), (secs * 4.0).sin()) * 255.0) as u8,
    );
}

// update state and re-render pieces
pub fn update_move(
    mut commands: Commands,
    mut state: ResMut<ChessState>,
    piece: Res<Piece>,
    mut mouse_ev: EventReader<MouseEvent>,
    mut query: Query<Entity, With<DrawnPiece>>,
    asset_server: Res<AssetServer>,
) {
    use MouseEvent::*;

    if let Some(PressChessboard(pos)) = mouse_ev.iter().next() {
        // if we cant find the move
        if state.moves.iter().find(|m| m.origin == piece.position && m.target == *pos).is_none() {
            return;
        }

        //dbg!(&state);

        //let PressChessboard(pos) =
        state.excecute_move(*piece, *pos);

        // despawn the pieces
        commands.entity(query.single_mut()).despawn_recursive();

        // re-spawn the pieces
        state.render_pieces(&mut commands, &asset_server)
    }
}
