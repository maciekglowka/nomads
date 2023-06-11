use bevy::prelude::*;
use bevy::ecs::system::Command;

use crate::common::components::Position;
use crate::hex::Hex;

use super::Player;

pub struct PlaceWorker{
    pub hex: Hex
}
impl Command for PlaceWorker {
    fn write(self, world: &mut World) {
        let Some(player) = world.get_resource::<Player>() else { return };
        let Some(entity) = player.current_worker else { return };
        let Some(mut entity) = world.get_entity_mut(entity) else { return };
        entity.insert(Position(self.hex));

        let Some(mut player) = world.get_resource_mut::<Player>() else { return };
        player.current_worker = None;
    }
}