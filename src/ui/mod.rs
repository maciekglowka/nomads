use bevy::prelude::*;

use crate::states::GameState;

mod assets;
mod cursor;
mod elements;
mod events;
mod planning;
mod relocation;

pub use cursor::Cursor;

const FONT_SIZE: f32 = 24.;
const MENU_PADDING: Val = Val::Px(8.);

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameUiState>()
        .add_event::<events::MenuCloseEvent>()
        .add_startup_system(assets::load_assets)
            .add_systems((cursor::spawn_cursor, relocation_start)
                .in_schedule(OnEnter(GameState::Relocation))
            )
            .add_system(
                cursor::move_cursor
                .in_set(OnUpdate(GameUiState::Cursor))
            )
            .add_system(
                relocation::cursor_action
                .in_set(OnUpdate(GameState::Relocation))
            )
            .add_systems(
                (clear::<cursor::Cursor>, planning_end)
                .in_schedule(OnExit(GameState::Planning))
            )
            .add_systems(
                (planning::cursor_action, planning::planning_end)
                .in_set(OnUpdate(GameUiState::Cursor))
                .in_set(OnUpdate(GameState::Planning))
            )
            .add_systems(
                (
                    elements::select_menu::update_menu::<Entity>,
                    elements::select_menu::close_menu::<Entity>,
                    planning::on_close_menu
                )
                .in_set(OnUpdate(GameUiState::CursorMenu))
                .in_set(OnUpdate(GameState::Planning))
            )
            .add_system(
                clear::<elements::select_menu::SelectMenu<Entity>>
                    .in_schedule(OnExit(GameUiState::CursorMenu))
            );
    }
}

#[derive(Clone, Debug, Default, Hash, Eq, States, PartialEq)]
pub enum GameUiState {
    #[default]
    None,
    Cursor,
    CursorMenu
}

#[derive(Resource)]
pub struct UiAssets {
    pub cursor_texture: Handle<TextureAtlas>,
    pub font: Handle<Font>
}

fn relocation_start(
    mut next_state: ResMut<NextState<GameUiState>>
) {
    next_state.set(GameUiState::Cursor);
}

fn planning_end(
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