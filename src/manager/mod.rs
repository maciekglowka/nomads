use bevy::prelude::*;

use crate::common::components::{Camp, Position};
use crate::states::{GameState, MainState};

pub mod events;

pub struct ManagerPlugin;

impl Plugin for ManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<events::PlanningEndEvent>()
            .add_event::<events::CollectingEndEvent>()
            .add_event::<events::RelocatedEvent>()
            .add_system(game_start.in_schedule(OnEnter(MainState::Game)))
            .add_system(game_end.in_schedule(OnExit(MainState::Game)))
            .add_system(relocation_end.run_if(on_event::<events::RelocatedEvent>()))
            .add_system(planning_end.run_if(on_event::<events::PlanningEndEvent>()))
            .add_system(collecting_end.run_if(on_event::<events::CollectingEndEvent>()));
    }
}

fn game_start(
    mut next_state: ResMut<NextState<GameState>>
) {
    next_state.set(GameState::Relocation);
}

fn game_end(
    mut next_state: ResMut<NextState<GameState>>
) {
    next_state.set(GameState::None);
}

fn relocation_end(
    mut next_state: ResMut<NextState<GameState>>,
) {
    next_state.set(GameState::Planning);
}

fn planning_end(
    mut next_state: ResMut<NextState<GameState>>
) {
    next_state.set(GameState::Collecting);
}

fn collecting_end(
    mut next_state: ResMut<NextState<GameState>>
) {
    next_state.set(GameState::Relocation);
}