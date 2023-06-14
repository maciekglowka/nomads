use bevy::prelude::*;

use crate::common::components::{Camp, Position};
use crate::graphics::math::{hex_to_v2, move_towards};

const SPEED: f32 = 200.;

pub fn setup(mut commands: Commands) {
    let camera = Camera2dBundle::default();
    commands.spawn(camera);
}

pub fn update(
    mut query: Query<&mut Transform, With<Camera>>,
    camp_query: Query<&Position, With<Camp>>,
    time: Res<Time>
) {
    let Ok(position) = camp_query.get_single() else { return };
    let Ok(mut transform) = query.get_single_mut() else { return };
    let target = hex_to_v2(position.0).extend(transform.translation.z);
    let step = SPEED * time.delta_seconds();
    transform.translation = move_towards(transform.translation, target, step);
}