use bevy::prelude::*;
use rand::prelude::*;
use std::collections::HashMap;

use crate::hex::{Hex, DIRECTIONS};
use crate::ui::Cursor;

use crate::common::{
    components::{Tile, Position, insert_data_components},
    enums::{Goods, TileKind}
};
use crate::data::TileDataParam;
use super::{
    Board,
    events::ExpandBoardEvent
};

pub fn expand_board(
    mut commands: Commands,
    mut board: ResMut<Board>,
    data: TileDataParam,
    mut ev_board: EventReader<ExpandBoardEvent>
) {
    for ev in ev_board.iter() {
        let mut rng = thread_rng();
        for q in ev.0.q - ev.1..=ev.0.q + ev.1 {
            for r in ev.0.r - ev.1..=ev.0.r + ev.1 {
                let hex =  Hex::new(q, r);
                if board.tiles.contains_key(&hex) { continue };
                let kind = match rng.gen_range(0..=2) {
                    0 => TileKind::Plains,
                    1 => TileKind::Bush,
                    _ => TileKind::Forest
                };
                let entity = spawn_tile(&mut commands, hex, kind, &data);
                board.tiles.insert(hex, entity);
            }
        }

    }
}

fn spawn_tile(
    commands: &mut Commands,
    hex: Hex,
    kind: TileKind,
    data: &TileDataParam
) -> Entity {
    let mut ec = commands.spawn((
            Position(hex),
            Tile(kind)
        ));
    if let Some(data) = data.get(&kind) {
        insert_data_components(&mut ec, &data.components);
    }
    ec.id()
}
