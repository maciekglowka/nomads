use bevy::prelude::*;

use crate::common::components::Position;
use crate::player::{
    commands::{RelocateCamp}
};

use super::{GameUiState, UiAssets};
use super::cursor::Cursor;
use super::elements::select_menu::{SelectMenu, SelectMenuOption, draw_menu};
use super::events::MenuCloseEvent;

pub fn cursor_action(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    assets: Res<UiAssets>,
    mut next_state: ResMut<NextState<GameUiState>>
) {
    if !keys.just_pressed(KeyCode::Space) {
        return
    }
    let options = vec![
        SelectMenuOption::<()>::new("Relocate camp".into(), ())
    ];
    draw_menu(
        &mut commands,
        options,
        assets.as_ref()
    );
    next_state.set(GameUiState::CursorMenu);
}

pub fn on_close_menu(
    mut commands: Commands,
    cursor_query: Query<&Position, With<Cursor>>,
    mut ev_menu: EventReader<MenuCloseEvent>,
    mut next_state: ResMut<NextState<GameUiState>>
) {
    for ev in ev_menu.iter() {
        next_state.set(GameUiState::Cursor);
        if !ev.0 { continue };
        let Ok(position) = cursor_query.get_single() else { continue };
        commands.add(
            RelocateCamp { hex: position.0 }
        );
        break;
    }
}