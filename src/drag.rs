use engine::{
    chess::{index::Index, square::Square},
    rules::piece::Piece,
};

use crate::{
    board::{Board, ColoredSquares, SquareColor},
    *,
};

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
        Self {
            pos: ***pos,
            square: ***hovered,
        }
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
            // dbg!("click!");
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
        // dbg!("drag end!");
    }
    // update drag
    else if mouse_button.pressed(MouseButton::Left) && *drag && mouse_pos.is_changed() {
        drag_events.send(DragEvent::To(HoverPos::new(&mouse_pos, &hovered)));
        // dbg!("drag!!");
    }
    // check for drag
    else if mouse_button.pressed(MouseButton::Left) && let Some(pos) = &*start {
        if (pos.pos - **mouse_pos).length() > DRAG_THRESHOLD {
            drag_events.send(DragEvent::Start(HoverPos::new(&mouse_pos, &hovered)));
            drag_events.send(DragEvent::To(HoverPos::new(&mouse_pos, &hovered)));
            *drag = true;
            // dbg!("drag start!");
        }
    }
}

#[derive(Debug)]
pub enum MoveEvent {
    Select(Square),
    Move(Square, Square),
}

/// Moves a piece around
pub fn move_event_sender(
    mut click_events: EventReader<ClickEvent>,
    mut move_events: EventWriter<MoveEvent>,
    board: Res<Board>,
    mut selected: Local<Option<Square>>,
) {
    // click events
    if let Some(ClickEvent(pos)) = click_events.iter().next() && let Some(square) = pos.square {
        let piece = board.board()[square];

        match *selected  {
            Some(_selected) => {
                *selected = None;
                move_events.send(MoveEvent::Move(_selected, square));
                if board.get_info(piece).is_some() {
                    *selected = Some(square);
                    move_events.send(MoveEvent::Select(square))
                }
            },
            None => {
                if board.get_info(piece).is_some() {
                    *selected = Some(square);
                    move_events.send(MoveEvent::Select(square))
                }
            }
        }
    }
}

pub fn click_move(
    mut move_events: EventReader<MoveEvent>,
    // mut selected: Local<Option<Square>>,
    // mut moves: Local<Vec<Square>>,
    mut board: ResMut<Board>,
) {
    for event in move_events.iter() {
        // dbg!(&event);
        match event {
            MoveEvent::Move(from, to) => {
                let Some(f) = board.get_info(*from) else { error!("tried to move square that is not a piece"); return};
                let t = board.get_info(*to);
                
                if t.is_none() || t.is_some_and(|t| t.team != f.team) {
                    board.state.make_move(*from, *to);
                }
            },
            _ => {},
        }
    }
} 