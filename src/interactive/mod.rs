mod drag_and_drop;
mod highlight;
mod mouse_event;
mod window;

use bevy::prelude::Component;

#[derive(Component)]
pub struct TargetSquare;

#[derive(Component)]
pub struct SelectedSquare();
