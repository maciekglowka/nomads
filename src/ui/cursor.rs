use bevy::prelude::*;

use crate::common::components::{Camp, Position};
use crate::graphics::{math::hex_to_v3, TILE_SIZE, OVERLAY_Z};
use crate::hex::Hex;
use super::UiAssets;

#[derive(Component)]
pub struct Cursor;

pub fn move_cursor(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Position, &mut Transform), With<Cursor>>
) {
    let mut dir = None;
    let Ok((mut position, mut transform)) = query.get_single_mut() else { return };

    if keys.just_pressed(KeyCode::W) {
        dir = Some(Hex::new(0, 1));
    }
    if keys.just_pressed(KeyCode::S) {
        dir = Some(Hex::new(0, -1));
    }
    if keys.just_pressed(KeyCode::A) {
        dir = Some(Hex::new(-1, 0));
    }
    if keys.just_pressed(KeyCode::D) {
        dir = Some(Hex::new(1, 0));
    }
    // if keys.just_pressed(KeyCode::A) {
    //     dir = match position.0.q % 2 {
    //         0 => Some(Hex::new(-1, 1)),
    //         _ => Some(Hex::new(-1, 0))
    //     };
    // }
    // if keys.just_pressed(KeyCode::D) {
    //     dir = match position.0.q % 2 {
    //         0 => Some(Hex::new(1, 0)),
    //         _ => Some(Hex::new(1, -1))
    //     };
    // }
    if let Some(dir) = dir {
        position.0 += dir;
        transform.translation = hex_to_v3(position.0, OVERLAY_Z); 
    }
}

pub fn spawn_cursor(
    mut commands: Commands,
    assets: Res<UiAssets>,
    camp_query: Query<&Position, With<Camp>>
) {
    let hex = match camp_query.get_single() {
        Ok(p) => p.0,
        Err(_) => Hex::default()
    };

    let mut sprite = TextureAtlasSprite::new(0);
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
    let v = hex_to_v3(hex, OVERLAY_Z); 
    commands.spawn((
            Cursor,
            Position(hex),
            SpriteSheetBundle {
                sprite,
                texture_atlas: assets.cursor_texture.clone(),
                transform: Transform::from_translation(v),
                ..Default::default()
            }
        ));
}

// const DIR_KEY_MAPPING: [(KeyCode, Hex); 4] = [
//     (KeyCode::W, Hex::new(0, 1)), (KeyCode::S, Hex::new()),
//     (KeyCode::A, Vector2Int::LEFT), (KeyCode::D, Vector2Int::RIGHT),
// ];