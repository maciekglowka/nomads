use bevy::prelude::*;

use crate::common::components::{Position, Piece, Worker};
use super::{
    GraphicsAssets, TILE_SIZE, PIECE_Z,
    math::hex_to_v3
};

pub fn spawn_piece_renderer(
    mut commands: Commands,
    query: Query<(Entity, &Position, Option<&Worker>), (Added<Position>, With<Piece>)>,
    assets: Res<GraphicsAssets>
) {
    for (entity, position, worker) in query.iter() {
        // temporary TODO - take info from data module
        let idx = if worker.is_some() { 1 } else { 0 };
        let mut sprite = TextureAtlasSprite::new(idx);
        sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
        let v = hex_to_v3(position.0, PIECE_Z);
        commands.entity(entity)
            .insert(
                SpriteSheetBundle {
                    sprite,
                    texture_atlas: assets.textures[&"pieces"].clone(),
                    transform: Transform::from_translation(v),
                    ..Default::default()
                }
            );
    }
}

pub fn remove_piece_renderer(
    mut commands: Commands,
    mut removed: RemovedComponents<Position>,
    query: Query<&Piece>
) {
    for entity in removed.iter() {
        // check if it's a piece
        if query.get(entity).is_err() { continue};
        commands.entity(entity).remove::<SpriteSheetBundle>();
    }
}

pub fn  update_piece_renderer(
    mut query: Query<(&Position, &mut Transform), (With<Piece>, Changed<Position>)>
) {
    for (position, mut transform) in query.iter_mut() {
        let v = hex_to_v3(position.0, PIECE_Z);
        transform.translation = v;
    }
}