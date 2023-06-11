use bevy::prelude::*;

use crate::common::components::{Position, Piece};
use super::{
    GraphicsAssets, TILE_SIZE, PIECE_Z,
    math::hex_to_v3
};

pub fn spawn_piece_renderer(
    mut commands: Commands,
    query: Query<(Entity, &Position, &Piece), Added<Piece>>,
    assets: Res<GraphicsAssets>
) {
    for (entity, position, piece) in query.iter() {
        // only camp for now
        let mut sprite = TextureAtlasSprite::new(0);
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
