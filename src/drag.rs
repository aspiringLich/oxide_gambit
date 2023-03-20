use engine::chess::square::Square;

use crate::{board::Board, *};

#[derive(Resource, Deref, DerefMut, Default)]
pub struct MousePos(Vec2);

pub fn init(mut commands: Commands) {
    commands.init_resource::<MousePos>();
    commands.init_resource::<HoveredTile>();
}

pub fn update_mouse_pos(
    q_windows: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut mouse_pos: ResMut<MousePos>,
) {
    // get the camera info and transform
    let (camera, camera_transform) = q_camera.single();
    let Ok(window) = q_windows.get_single() else { error!("Error getting window"); return };

    // get the mouse position in world coordinates
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        if world_position == **mouse_pos {
            return;
        }

        **mouse_pos = world_position;
        // dbg!(world_position);
    }
}

#[derive(Resource, Deref, DerefMut, Default, Debug)]
pub struct HoveredTile(Option<Square>);

pub fn update_hovered_tile(mut hovered: ResMut<HoveredTile>, mouse_pos: Res<MousePos>) {
    if !mouse_pos.is_changed() {
        return;
    }

    let x = mouse_pos.x / board::TILE_SIZE + 4.0;
    let y = mouse_pos.y / board::TILE_SIZE + 4.0;

    let (x, y) = (x.floor() as i32, y.floor() as i32);

    match (x, y) {
        (0..=7, 0..=7) => {
            let square = Square::from_xy(x, y).unwrap();
            if Some(square) == **hovered {
                return;
            }
            **hovered = Some(square);
            // dbg!(&hovered);
        }
        _ => **hovered = None,
    }
}

pub fn highlight_square(mut commands: Commands, hovered: Res<HoveredTile>, board: Res<Board>) {
    
}
