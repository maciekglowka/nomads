use bevy::prelude::*;
use bevy::ecs::system::Command;

use crate::common::components::{Camp, Position};
use crate::hex::Hex;
use crate::manager::events::RelocatedEvent;

use super::Player;

pub struct PlaceWorker {
    pub entity: Entity,
    pub hex: Hex
}
impl Command for PlaceWorker {
    fn write(self, world: &mut World) {
        let Some(player) = world.get_resource::<Player>() else { return };
        let Some(mut entity) = world.get_entity_mut(self.entity) else { return };
        entity.insert(Position(self.hex));
    }
}

pub struct RemoveWorker {
    pub entity: Entity
}
impl Command for RemoveWorker {
    fn write(self, world: &mut World) {
        let Some(mut entity) = world.get_entity_mut(self.entity) else { return };
        entity.remove::<Position>();
    }
}

pub struct RelocateCamp {
    pub hex: Hex
}
impl Command for RelocateCamp {
    fn write(self, world: &mut World) {
        let Ok(mut position) = world.query_filtered::<&mut Position, With<Camp>>()
            .get_single_mut(world) else { return };
        position.0 = self.hex;
        world.send_event(RelocatedEvent);
    }
}