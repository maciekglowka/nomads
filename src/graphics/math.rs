use bevy::prelude::*;

use crate::hex::Hex;

use super::{HEIGHT_RATIO, TILE_SIZE};

pub fn hex_to_v2(hex: Hex) -> Vec2 {
    Vec2::new(
        TILE_SIZE * 3./4. * hex.q as f32,
        TILE_SIZE * HEIGHT_RATIO * (0.5 * hex.q as f32 + hex.r as f32)
    )
}
pub fn hex_to_v3( hex: Hex, z: f32) -> Vec3 {
    let relative_z = z - 0.1 * hex.r as f32 - 0.04 * hex.q as f32;
    hex_to_v2(hex).extend(relative_z)
}

pub fn move_towards(origin: Vec3, target: Vec3, max_delta: f32) -> Vec3 {
    let a = target - origin;
    let l = a.length();
    if (l <= max_delta || l == 0.) {
        return target
    }
    origin + a / l * max_delta
} 