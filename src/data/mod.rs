use bevy::{
    asset::{Asset, AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    utils::BoxedFuture
};
use serde::Deserialize;
use serde_yaml;
use std::{
    marker::PhantomData
};

mod tiles;
pub use tiles::TileDataParam;

pub struct DataPlugin;
impl Plugin for DataPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DataAssets>()
            .add_asset::<tiles::TileAsset>()
            .add_asset_loader(
                YamlLoader::<tiles::TileAsset>(
                    PhantomData::<tiles::TileAsset>, &["tiles.yaml"]
                )
            )
            .add_startup_system(tiles::load_data);
    }
}

pub struct YamlLoader<T>(PhantomData<T>, &'static [&'static str]);

impl<D> AssetLoader for YamlLoader<D>
where for<'de> D : Deserialize<'de> + Asset {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let asset = serde_yaml::from_slice::<D>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(asset));
            Ok(())
        })
    }
    fn extensions(&self) -> &[&str] {
        self.1
    }
}

#[derive(Default, Resource)]
pub struct DataAssets {
    pub tile_data: Handle<tiles::TileAsset>,
}

#[derive(Deserialize)]
pub struct SpriteData {
    pub atlas: String,
    pub index: usize
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct RGB(u8, u8, u8);

impl Into<Color> for RGB {
    fn into(self) -> Color {
        Color::rgb_u8(self.0, self.1, self.2)
    }
}