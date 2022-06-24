use crate::{
    chess_logic::{ChessMove, ChessState},
    interactive::update_move,
    render::DrawnPiece,
};
use bevy::{prelude::*, tasks::AsyncComputeTaskPool};
use futures_lite::future;

use super::*;

/// spawn an async task to calculate the next chess move
pub fn spawn_calc_task(
    state: &ChessState,
    commands: &mut Commands,
    thread_pool: Res<AsyncComputeTaskPool>,
) {
    if DEBUG {
        eprintln!("Started Compute Task...");
    }
    let copy = state.clone();
    let task = thread_pool.spawn(async move { copy.run_minimax(4) });
    commands.spawn().insert(ComputeMove(task));
}

/// spawn an async task to calculate the next chess move
pub fn excecute_calc_task(
    mut state: ResMut<ChessState>,
    mut commands: Commands,
    mut task_query: Query<(Entity, &mut ComputeMove)>,
    mut query: Query<Entity, With<DrawnPiece>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok((entity, mut task)) = task_query.get_single_mut() {
        let ComputeMove(task) = &mut *task;
        if let Some(chess_move) = future::block_on(future::poll_once(&mut *task)) {
            if DEBUG {
                eprintln!("Compute Task Finished!");
            }

            commands.entity(entity).despawn();
            update_move(&mut commands, state.as_mut(), query, asset_server, chess_move);
        }
    }
}
