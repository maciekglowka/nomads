use bevy::prelude::*;

use crate::common::components::{Camp, Piece, Position, Worker, Supply};
use crate::hex::Hex;
use crate::manager::events::CollectingEndEvent;

use super::{CollectedGoods, Collecting, Player};

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
    for _ in 0..2 {
        let entity = commands.spawn((
                Piece,
                Worker { name: "Stefan".into( )}
            ))
            .id();
        player.workers.push(entity);
    }
    // player.current_worker = Some(entity);
}

pub fn remove_workers(
    mut commands: Commands,
    query: Query<Entity, With<Worker>>
) {
    // remove workers from the board
    for entity in query.iter() {
        commands.entity(entity).remove::<Position>();
    }
}

pub fn collecting_start(
    mut commands: Commands,
    mut goods: ResMut<CollectedGoods>,
    worker_query: Query<Entity, With<Worker>>
) {
    goods.0 = Vec::new();

    for entity in worker_query.iter() {
        // mark all the workers read
        commands.entity(entity).insert(Collecting);
    }
}

pub fn worker_collect(
    mut commands: Commands,
    mut goods: ResMut<CollectedGoods>,
    worker_query: Query<(Entity, &Position), (With<Worker>, With<Collecting>)>,
    supply_query: Query<(&Position, &Supply)>,
    mut ev_collect: EventWriter<CollectingEndEvent>
) {
    for (entity, position) in worker_query.iter() {
        commands.entity(entity).remove::<Collecting>();

        let supplier = supply_query.iter()
            .find(|(p, _)| p.0 == position.0);

        let Some((_, supply)) = supplier else { continue };
        goods.0 = supply.0.iter()
            .map(|(k, v)| (*k, *v as i32))
            .collect();

        // select only one at a time
        return;
    }
    // if we get here it means there is no workers left
    ev_collect.send(CollectingEndEvent);
}

pub fn apply_collect(
    mut player: ResMut<Player>,
    mut goods: ResMut<CollectedGoods>,
) {
    for (kind, quantity) in goods.0.iter() {
        let cur = player.goods.entry(*kind).or_insert(0);
        *cur += *quantity as u32;
    }
    goods.0 = Vec::new();
}