use bevy::prelude::*;
use std::collections::HashMap;

use crate::hex::{DIRECTIONS, Hex};
use crate::states::MainState;

pub mod events;
mod systems;

pub use crate::common::components::{Position, Tile};

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Board>()
            .add_event::<events::ExpandBoardEvent>()
            .add_system(systems::expand_board
                .run_if(on_event::<events::ExpandBoardEvent>())
            );
    }
}

#[derive(Default, Resource)]
pub struct Board {
    pub tiles: HashMap<Hex, Entity>,
}
