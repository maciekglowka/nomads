use bevy::prelude::*;
use rand::prelude::*;
use std::collections::HashMap;

use crate::globals::BOARD_RADIUS;
use crate::hex::{Hex, DIRECTIONS};
use crate::ui::Cursor;

use crate::common::{
    components::{Tile, Position, insert_data_components},
    enums::{Goods, TileKind}
};
use crate::data::TileDataParam;
use super::{
    Board,
};

pub fn spawn_board(
    mut commands: Commands,
    mut board: ResMut<Board>,
    data: TileDataParam
) {
    let mut rng = thread_rng();
    
    for q in -BOARD_RADIUS..BOARD_RADIUS {
        for r in -BOARD_RADIUS..BOARD_RADIUS {
            let kind = match rng.gen_range(0..=2) {
                0 => TileKind::Plains,
                1 => TileKind::Bush,
                _ => TileKind::Forest
            };
            spawn_tile(&mut commands, Hex::new(q, r), kind, &data);
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
