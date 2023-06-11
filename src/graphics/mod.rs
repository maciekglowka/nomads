use bevy::prelude::*;
use std::collections::HashMap;

mod assets;
pub mod math;
mod pieces;
mod tiles;

pub const HEIGHT_RATIO: f32 = 17. / 32.;
pub const TILE_SIZE: f32 = 128.;
pub const TILE_Z: f32 = 50.;
pub const PIECE_Z: f32 = 100.;
pub const OVERLAY_Z: f32 = 200.;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(assets::load_assets)
            .add_system(pieces::spawn_piece_renderer)
            .add_system(tiles::spawn_tile_renderer);
    }
}

#[derive(Resource)]
pub struct GraphicsAssets {
    pub textures: HashMap<&'static str, Handle<TextureAtlas>>
}
