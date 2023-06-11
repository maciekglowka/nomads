use bevy::prelude::*;

mod assets;
pub mod math;
mod tiles;

pub const HEIGHT_RATIO: f32 = 17. / 32.;
pub const TILE_SIZE: f32 = 128.;
pub const TILE_Z: f32 = 50.;
pub const OVERLAY_Z: f32 = 200.;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(assets::load_assets)
            .add_system(tiles::spawn_tile_renderer);
    }
}

#[derive(Resource)]
pub struct GraphicsAssets {
    pub tiles_texture: Handle<TextureAtlas>
}
