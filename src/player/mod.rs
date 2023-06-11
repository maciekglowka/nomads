use bevy::prelude::*;

use crate::states::MainState;

mod systems;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            systems::spawn_camp.in_schedule(OnEnter(MainState::Game))
        );
    }
}

