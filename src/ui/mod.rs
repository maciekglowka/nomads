use bevy::prelude::*;

use crate::states::MainState;

mod assets;
mod cursor;
mod events;
mod workers;

pub use cursor::Cursor;

const FONT_SIZE: f32 = 24.;
const MENU_PADDING: Val = Val::Px(8.);

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameUiState>()
        .add_event::<events::MenuCloseEvent>()
        .add_startup_system(assets::load_assets)
            .add_systems(
                (cursor::spawn_cursor, game_start)
                .in_schedule(OnEnter(MainState::Game))
            )
            .add_systems(
                (clear::<cursor::Cursor>, game_end)
                .in_schedule(OnExit(MainState::Game))
            )
            .add_systems(
                (cursor::cursor_input, workers::place_worker)
                .in_set(OnUpdate(GameUiState::Cursor))
            );
    }
}

#[derive(Clone, Debug, Default, Hash, Eq, States, PartialEq)]
pub enum GameUiState {
    #[default]
    None,
    Cursor,
    // BuildMenu
}

#[derive(Resource)]
pub struct UiAssets {
    pub cursor_texture: Handle<TextureAtlas>,
    pub font: Handle<Font>
}

fn game_start(
    mut next_state: ResMut<NextState<GameUiState>>
) {
    next_state.set(GameUiState::Cursor);
}

fn game_end(
    mut next_state: ResMut<NextState<GameUiState>>
) {
    next_state.set(GameUiState::None);
}

fn clear<T: Component> (
    mut commands: Commands,
    query: Query<Entity, With<T>>
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}