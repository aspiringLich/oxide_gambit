use engine::{
    chess::{index::Index, square::Square},
    rules::piece::Piece,
};

use crate::{
    board::{Board, Decoration, PiecePos, Selectable, TILE_SIZE},
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

#[derive(Debug)]
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

#[derive(Debug)]
pub enum DragEvent {
    Start(HoverPos),
    To(HoverPos),
    End(HoverPos),
}

pub struct MoveEvent {
    pub from: Square,
    pub to: Square,
    pub verified: bool,
}

impl MoveEvent {
    fn new(from: Square, to: Square, verified: bool) -> Self {
        Self { from, to, verified }
    }
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
        *drag = false;
        // dbg!("drag end!");
    }
    // update drag
    else if mouse_button.pressed(MouseButton::Left) && *drag && mouse_pos.is_changed() {
        drag_events.send(DragEvent::To(HoverPos::new(&mouse_pos, &hovered)));
        // dbg!("drag!!");
    }
    // check for drag
    else if mouse_button.pressed(MouseButton::Left) && let Some(pos) = &*start && !*drag {
        if (pos.pos - **mouse_pos).length() > DRAG_THRESHOLD {
            drag_events.send(DragEvent::Start(HoverPos::new(&mouse_pos, &hovered)));
            // drag_events.send(DragEvent::To(HoverPos::new(&mouse_pos, &hovered)));
            *drag = true;
            // dbg!("drag start!");
        }
    }
}

pub fn do_move_events(mut move_events: EventReader<MoveEvent>, mut board: ResMut<Board>) {
    for event in move_events.iter() {
        let MoveEvent {
            from,
            to,
            mut verified,
        } = event;
        let (from, to) = (*from, *to);

        // dbg!("dkajfa");
        if !verified {
            let Some(from_i) = board.get_info(from) else { error!("tried to move square that is not a piece"); return};
            let to_i = board.get_info(to);

            if to_i.is_none() || to_i.is_some_and(|t| t.team != from_i.team) {
                let piece = board.board()[from];
                let moves = board.state.moves.filter(piece).collect::<Vec<_>>();

                if moves.iter().any(|m| m.to == to) {
                    verified = true
                }
            }
        }

        if verified {
            board.state.make_move(from, to);
        }
    }
}

pub fn select(
    mut click_events: EventReader<ClickEvent>,
    mut drag_events: EventReader<DragEvent>,
    mut selected: Local<Option<Square>>,
    board: Res<Board>,
    mut decorations: EventWriter<Decoration>,
    mut move_events: EventWriter<MoveEvent>,
    selectable: Res<Selectable>,
) {
    if board.is_changed() {
        *selected = None;
    }
    macro reset() {
        *selected = None;
        decorations.send(Decoration::Clear);
    }

    for event in drag_events.iter() {
        use DragEvent::*;
        match event {
            Start(pos) if let Some(square) = pos.square && selectable[*square as usize] => {
                *selected = Some(square);
                let piece = board.board()[square];
                if selectable[*square as usize] == false {
                    reset!();
                    return;
                }

                let moves = board.state.moves.filter(piece).collect::<Vec<_>>();

                decorations.send(Decoration::Clear);
                decorations.send(Decoration::Highlight(square));
                if !moves.is_empty() {
                    for m in moves {
                        decorations.send(Decoration::Move(m.to));
                    }
                }
            }
            End(_) => {
                // *selected = None;
            }
            _ => {}
        }
    }

    for ClickEvent(square) in click_events.iter() {
        let Some(to) = square.square else { continue };
        // dbg!(&event);
        if let Some(from) = *selected {
            if from == to {
                reset!();
                continue;
            }

            let Some(f) = board.get_info(from) else { error!("tried to move square that is not a piece"); return};
            let t = board.get_info(to);

            if t.is_none() || t.is_some_and(|t| t.team != f.team) {
                let piece = board.board()[from];
                let moves = board.state.moves.filter(piece).collect::<Vec<_>>();

                if moves.iter().any(|m| m.to == to) {
                    move_events.send(MoveEvent::new(from, to, true));
                    *selected = None;
                    continue;
                } else {
                    reset!();
                }
            } else {
                reset!();
            }
        }

        // select piece
        if selected.is_some_and(|s| s == to) {
            reset!();
            return;
        }
        *selected = Some(to);

        let piece = board.board()[to];
        if selectable[*to as usize] == false {
            reset!();
            return;
        }

        let moves = board.state.moves.filter(piece).collect::<Vec<_>>();

        decorations.send(Decoration::Clear);
        decorations.send(Decoration::Highlight(to));
        if !moves.is_empty() {
            for m in moves {
                decorations.send(Decoration::Move(m.to));
            }
        }
    }
}

pub fn drag(
    mut move_events: EventWriter<MoveEvent>,
    mut drag_events: EventReader<DragEvent>,
    mut selected: Local<Option<Entity>>,
    mut pos: Local<Vec3>,
    mut start_mouse_pos: Local<Vec2>,
    mut from: Local<Square>,
    mut q: ParamSet<(
        Query<(Entity, &Transform, &PiecePos)>,
        Query<&mut Transform>,
    )>,
    board: Res<Board>,
    selectable: Res<Selectable>,
    // mouse_pos: Res<MousePos>,
) {
    if board.is_changed() {
        *selected = None;
    }

    for drag in drag_events.iter() {
        use DragEvent::*;
        // dbg!(&drag);
        match drag {
            Start(mouse_pos) if let Some(square) = mouse_pos.square && selectable[*square as usize]=> {
                let q_piece_pos = q.p0();
                // dbg!(mouse_pos);
                let Some((e, transform, p)) = q_piece_pos.iter().find(|(_, _, p)| ***p == mouse_pos.square.unwrap()) else { error!("Could not find dragged piece"); return};
                *from = **p;
                *selected = Some(e);
                *pos = transform.translation;
                *start_mouse_pos = mouse_pos.pos;
            }
            To(mouse_pos) if let Some(e) = *selected => {
                let mut q_transform = q.p1();
                let mut transform = q_transform.get_mut(e).unwrap();
                // let delta = mouse_pos.pos - *start_mouse_pos;
                transform.translation = mouse_pos.pos.extend(16.0);
                transform.translation.y += 4.0;
            }
            End(mouse_pos) => {
                let mut q_transform = q.p1();
                if let Some(e) = *selected {
                    let mut transform = q_transform.get_mut(e).unwrap();
                    transform.translation = *pos;
                    move_events.send(MoveEvent::new(*from, mouse_pos.square.unwrap(), false));
                }
                *selected = None;
            }
            _ => {}
        }
    }
}
