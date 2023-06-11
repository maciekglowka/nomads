use bevy::prelude::*;

use super::GraphicsAssets;

const TILES_PATH: &str = "tiles/tiles.png";
const HEX_PATH: &str = "hex.png";

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlasses: ResMut<Assets<TextureAtlas>>,
    mut asset_list: ResMut<crate::assets::AssetList>
) {
    let tiles_texture = asset_server.load(TILES_PATH);
    asset_list.0.push(tiles_texture.clone_untyped());
    let tiles_atlas = TextureAtlas::from_grid(
        tiles_texture,
        Vec2::splat(32.),
        4,
        4,
        None,
        None
    );
    let tiles_handle = texture_atlasses.add(tiles_atlas);

    commands.insert_resource(
        GraphicsAssets { 
            tiles_texture: tiles_handle
        }
    );
}