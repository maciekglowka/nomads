use bevy::prelude::*;
use bevy::ecs::system::Command;

use crate::common::components::Position;
use crate::hex::Hex;

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