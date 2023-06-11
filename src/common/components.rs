use bevy::prelude::*;
use bevy::ecs::system::EntityCommands;
use serde::Deserialize;
use serde_yaml;

use crate::hex::Hex;

use super::enums::{Goods, TileKind};


// pub trait GameComponent: Bundle {
//     fn description(&self) -> String { "".into() }
// }

// all the game logic components that can be deserialized
// from YAML data - to build game objects' properties

#[derive(Component, Deserialize)]
pub struct CampSite;

#[derive(Component, Deserialize)]
pub struct Supply(Vec<Goods>);
// impl GameComponent for Supply {
//     fn description(&self) -> String {
//         format!("Supplies")
//     }
// }
#[derive(Component, Deserialize)]
pub struct Upgrade;


// other common components that are attached depending on the context

#[derive(Component)]
pub struct Camp;

#[derive(Component)]
pub struct Piece;

#[derive(Component)]
pub struct Position(pub Hex);

#[derive(Component)]
pub struct Tile(pub TileKind);

// helper fns

pub fn insert_data_components(object: &mut EntityCommands, value: &serde_yaml::Value) {
    let Some(component_data) = value.as_mapping() else { return };

    for (name, data) in component_data.iter() {
        let Some(name) = name.as_str() else { continue };
        match name {
            "CampSite" => insert_single::<CampSite>(object, data),
            "Supply" => insert_single::<Supply>(object, data),
            _ => continue
        };
    }
}

fn insert_single<T>(object: &mut EntityCommands, data: &serde_yaml::Value)
where for<'de> T: Bundle + Deserialize<'de> {
    let Ok(component) = serde_yaml::from_value::<T>(data.clone()) else { return};
    object.insert(component);
}
