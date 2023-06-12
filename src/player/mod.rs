use bevy::prelude::*;

use crate::states::MainState;

pub mod commands;
mod systems;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Player>()
            .add_systems(
            (systems::spawn_camp, systems::spawn_worker)
            .in_schedule(OnEnter(MainState::Game))
        );
    }
}

#[derive(Default, Resource)]
pub struct Player {
    pub workers: Vec<Entity>
}
