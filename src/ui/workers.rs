use bevy::prelude::*;

use crate::common::components::Position;
use crate::player::commands::PlaceWorker;

use super::cursor::Cursor;

pub fn place_worker(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    cursor_query: Query<&Position, With<Cursor>>
) {
    if !keys.just_pressed(KeyCode::Space) { return };
    let Ok(cursor) = cursor_query.get_single() else { return };
    commands.add(
        PlaceWorker { hex: cursor.0 }
    );
}