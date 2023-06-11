use bevy::prelude::*;

use crate::common::components::{Camp, Piece, Position};
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