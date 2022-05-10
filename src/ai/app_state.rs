use crate::chess_logic::{ChessMove, ChessState};
use bevy::prelude::{ResMut, State};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    PlayerTurnHold,
    PlayerTurn,
    AiTurnHold,
    AiTurn,
}

/*
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

let (sender, receiver) = channel();

thread::spawn(move || {
    thread::sleep(Duration::from_secs(1));
    sender.send(1).unwrap();
    sender.send(2).unwrap();
    sender.send(3).unwrap();
    println!("whee #1");
});

if !receiver.try_iter().next().is_none()
*/
