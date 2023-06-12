use bevy::prelude::*;

mod assets;
mod board;
mod camera;
mod common;
mod data;
mod globals;
mod graphics;
mod hex;
mod manager;
mod player;
mod states;
mod ui;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(
                WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (
                            globals::WINDOW_WIDTH,
                            globals::WINDOW_HEIGHT
                        ).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }
            ).set(
                ImagePlugin::default_nearest()
            )
        )
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::BLACK))
        .add_state::<states::MainState>()
        .add_state::<states::GameState>()
        .add_plugin(assets::AssetsPlugin)
        .add_plugin(board::BoardPlugin)
        .add_plugin(data::DataPlugin)
        .add_plugin(graphics::GraphicsPlugin)
        .add_plugin(manager::ManagerPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(ui::UiPlugin)
        .add_startup_system(camera::setup)
        .run();
}
