use bevy::prelude::*;
use std::collections::HashMap;

use crate::board::events::ExpandBoardEvent;
use crate::common::components::{Camp, Consume, Piece, Position, Worker, Supply};
use crate::common::enums::Goods;
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

pub fn expand_board(
    mut ev_board: EventWriter<ExpandBoardEvent>,
    camp_query: Query<&Position, (With<Camp>, Changed<Position>)>
) {
    let Ok(position) = camp_query.get_single() else { return };
    ev_board.send(ExpandBoardEvent(position.0, 4));
}

pub fn spawn_worker(
    mut commands: Commands,
    mut player: ResMut<super::Player>
) {
    for i in 0..2 {
        let entity = commands.spawn((
                Consume(HashMap::from_iter([(Goods::Food, 4), (Goods::Energy, 3)])),
                Piece,
                Worker { name: format!("Stefan {}", i)}
            ))
            .id();
        player.workers.push(entity);
    }
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

pub fn worker_craft(
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

pub fn consume_goods(
    mut player: ResMut<Player>,
    consumer_query: Query<&Consume>
) {
    for consume in consumer_query.iter() {
        for (kind, quantity) in consume.0.iter() {
            let cur = player.goods.entry(*kind).or_insert(0);
            *cur = cur.saturating_sub(*quantity);
        }
    }
}