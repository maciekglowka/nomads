use bevy::prelude::*;

use crate::common::components::{Camp, Piece, Position, Worker};
use crate::hex::Hex;

pub fn spawn_camp(
    mut commands: Commands
) {
    commands.spawn((
        Camp,
        Piece,
        Position(Hex::new(0, 0))
    ));
}

pub fn spawn_worker(
    mut commands: Commands,
    mut player: ResMut<super::Player>
) {
    let entity = commands.spawn((
            Piece,
            Worker
        ))
        .id();
    player.current_worker = Some(entity);
}