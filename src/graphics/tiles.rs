use bevy::prelude::*;

use crate::common::components::{Position, Tile};
use super::{
    GraphicsAssets, TILE_SIZE, TILE_Z, HEIGHT_RATIO,
    math::hex_to_v3
};

pub fn spawn_tile_renderer(
    mut commands: Commands,
    query: Query<(Entity, &Position, &Tile), Added<Tile>>,
    assets: Res<GraphicsAssets>,
    data: crate::data::TileDataParam
) {
    for (entity, position, tile) in query.iter() {
        let Some(tile_data) = data.get(&tile.0) else { continue };
        let mut sprite = TextureAtlasSprite::new(tile_data.sprite);
        sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
        sprite.color = Color::WHITE;
        let v = hex_to_v3(position.0, TILE_Z);
        commands.entity(entity)
            .insert(
                SpriteSheetBundle {
                    sprite,
                    texture_atlas: assets.tiles_texture.clone(),
                    transform: Transform::from_translation(v),
                    ..Default::default()
                }
            );
    }
}