use bevy::prelude::*;
use std::collections::HashMap;

use super::GraphicsAssets;

const TEXTURES: [(&str, &str, f32, usize, usize); 2] = [
    ("tiles", "tiles/tiles.png", 32., 4, 4),
    ("pieces", "tiles/pieces.png", 32., 4, 4)
];


pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlasses: ResMut<Assets<TextureAtlas>>,
    mut asset_list: ResMut<crate::assets::AssetList>
) {
    let mut textures = HashMap::new();
    for texture in TEXTURES {
        let img = asset_server.load(texture.1);
        asset_list.0.push(img.clone_untyped());
        let atlas = TextureAtlas::from_grid(
            img,
            Vec2::splat(texture.2),
            texture.3,
            texture.4,
            None,
            None
        );
        let handle = texture_atlasses.add(atlas);
        textures.insert(texture.0, handle);
    }

    commands.insert_resource(
        GraphicsAssets { textures }
    );
}