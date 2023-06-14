use bevy::prelude::*;
use std::collections::HashMap;

use crate::common::enums::Goods;
use crate::states::{GameState, MainState};

pub mod commands;
mod systems;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Player>()
            .init_resource::<CollectedGoods>()
            .configure_sets((
                CollectingSet::Collect,
                CollectingSet::Modify,
                CollectingSet::Apply
            ).chain().in_set(OnUpdate(GameState::Collecting)))
            .add_systems(
                (systems::spawn_camp, systems::spawn_worker)
                .in_schedule(OnEnter(MainState::Game))
            )
            .add_system(
                systems::expand_board
                .in_set(OnUpdate(MainState::Game))
            )
            .add_system(systems::remove_workers
                .in_schedule(OnExit(GameState::Relocation))
            )
            .add_system(systems::collecting_start
                .in_schedule(OnEnter(GameState::Collecting))
            )
            .add_system(systems::worker_collect
                .in_set(CollectingSet::Collect)
            )
            .add_system(systems::apply_collect
                .in_set(CollectingSet::Apply)
            )
            .add_system(systems::consume_goods
                .in_schedule(OnExit(GameState::Collecting))
            );
    }
}

#[derive(Default, Resource)]
pub struct Player {
    pub workers: Vec<Entity>,
    pub goods: HashMap<Goods, u32>
}

#[derive(Default, Resource)]
// allows temporarily to go negative
pub struct CollectedGoods(pub Vec<(Goods, i32)>);

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum CollectingSet {
    Collect,
    Modify,
    Apply
}

#[derive(Component)]
// temporary apply to workers to mark that they're ready to collect goods
pub struct Collecting;