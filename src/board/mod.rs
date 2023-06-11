use bevy::prelude::*;
use std::collections::HashMap;

use crate::hex::{DIRECTIONS, Hex};
use crate::states::MainState;

mod systems;

pub use crate::common::components::{Position, Tile};

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Board>()
            .add_system(systems::spawn_board.in_schedule(OnEnter(MainState::Game)));
    }
}

#[derive(Default, Resource)]
pub struct Board {
    pub tiles: HashMap<Hex, Entity>,
}
