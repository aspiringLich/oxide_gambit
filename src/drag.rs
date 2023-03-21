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

/// The tile the mouse is currently hovered over
#[derive(Resource, Deref, DerefMut, Default, Debug)]
pub struct HoveredTile(Option<Square>);

/// Updates the tile the mouse is currently hovered over
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

pub struct HoverPos {
    pub pos: Vec2,
    pub square: Option<Square>,
}

impl HoverPos {
    pub fn new(pos: &Res<MousePos>, hovered: &Res<HoveredTile>) -> Self {
        Self { pos: ***pos, square: ***hovered }
    }
}

#[derive(Deref, DerefMut)]
pub struct ClickEvent(HoverPos);

pub enum DragEvent {
    Start(HoverPos),
    To(HoverPos),
    End(HoverPos),
}

const DRAG_THRESHOLD: f32 = 1.0;

/// Click events are sent when the mouse is pressed and released without moving too much
pub fn click_event_sender(
    mut click_events: EventWriter<ClickEvent>,
    mouse_button: Res<Input<MouseButton>>,
    mouse_pos: Res<MousePos>,
    hovered: Res<HoveredTile>,
    mut start: Local<Option<HoverPos>>,
) {
    if mouse_button.just_pressed(MouseButton::Left)  {
        *start = Some(HoverPos::new(&mouse_pos, &hovered));
    }
    // check for click
    else if mouse_button.just_released(MouseButton::Left) && let Some(pos) = &*start {
        if (pos.pos - **mouse_pos).length() < DRAG_THRESHOLD {
            click_events.send(ClickEvent(HoverPos::new(&mouse_pos, &hovered)));
            dbg!("click!");
        }
    }
    
    // reset start when cursor moves too far
    if mouse_button.pressed(MouseButton::Left) && let Some(pos) = &*start {
        if (pos.pos - **mouse_pos).length() > DRAG_THRESHOLD {
            *start = None;
        }
    } 
}

/// Drag events are sent when the mouse is pressed and moved.
pub fn drag_event_sender(
    mut drag_events: EventWriter<DragEvent>,
    mouse_button: Res<Input<MouseButton>>,
    mouse_pos: Res<MousePos>,
    hovered: Res<HoveredTile>,
    mut start: Local<Option<HoverPos>>,
    mut drag: Local<bool>,
) {
    // start tracking mouse
    if mouse_button.just_pressed(MouseButton::Left)  {
        *start = Some(HoverPos::new(&mouse_pos, &hovered));
    }
    // stop drag
    else if mouse_button.just_released(MouseButton::Left) && *drag {
        drag_events.send(DragEvent::End(HoverPos::new(&mouse_pos, &hovered)));
        dbg!("drag end!");
    }
    // update drag
    else if mouse_button.pressed(MouseButton::Left) && *drag && mouse_pos.is_changed() {
        drag_events.send(DragEvent::To(HoverPos::new(&mouse_pos, &hovered)));
        dbg!("drag!!");
    }
    // check for drag
    else if mouse_button.pressed(MouseButton::Left) && let Some(pos) = &*start {
        if (pos.pos - **mouse_pos).length() > DRAG_THRESHOLD {
            drag_events.send(DragEvent::Start(HoverPos::new(&mouse_pos, &hovered)));
            drag_events.send(DragEvent::To(HoverPos::new(&mouse_pos, &hovered)));
            *drag = true;
            dbg!("drag start!");
        }
    }
}
