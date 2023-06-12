use bevy::prelude::*;

use crate::common::components::{Position, Worker};
use crate::manager::events::PlanningEndEvent;
use crate::player::{
    commands::{PlaceWorker, RemoveWorker},
    Player
};

use super::{GameUiState, UiAssets};
use super::cursor::Cursor;
use super::elements::select_menu::{SelectMenu, SelectMenuOption, draw_menu};
use super::events::MenuCloseEvent;


pub fn on_close_menu(
    mut commands: Commands,
    mut ev_menu: EventReader<MenuCloseEvent>,
    cursor_query: Query<&Position, With<Cursor>>,
    menu_query: Query<&SelectMenu<Entity>>,
    mut next_state: ResMut<NextState<GameUiState>>
) {
    for ev in ev_menu.iter() {
        next_state.set(GameUiState::Cursor);
        if !ev.0 { continue };
        let Ok(menu) = menu_query.get_single() else { continue };
        let Ok(position) = cursor_query.get_single() else { continue };
        let option = menu.get_current();
        commands.add(PlaceWorker { entity: option.value, hex: position.0 });
        break;
    }
}

pub fn cursor_action(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    placed_query: Query<(Entity, &Position), With<Worker>>,
    notplaced_query: Query<&Worker, Without<Position>>,
    player: Res<Player>,
    cursor_query: Query<&Position, With<Cursor>>,
    assets: Res<UiAssets>,
    mut next_state: ResMut<NextState<GameUiState>>
) {
    if !keys.just_pressed(KeyCode::Space) {
        return
    }
    let Ok(cursor) = cursor_query.get_single() else { return };
    let worker = placed_query.iter()
        .find(|(_, p)| p.0 == cursor.0);
    if let Some((entity, _)) = worker {
        commands.add(RemoveWorker { entity });
        return;
    }
    let available = player.workers.iter()
        .filter_map(|e| match notplaced_query.get(*e) {
            Ok(w) => Some((w, e)),
            _ => None
        })
        .map(|(w, e)| SelectMenuOption::<Entity>::new(w.name.clone(), *e))
        .collect::<Vec<_>>();
    if available.len() == 0 { return };
    draw_menu(
        &mut commands,
        available,
        assets.as_ref()
    );
    next_state.set(GameUiState::CursorMenu);
}


pub fn planning_end(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    mut ev_planning: EventWriter<PlanningEndEvent>
) {
    if !keys.just_pressed(KeyCode::Return) {
        return
    };
    ev_planning.send(PlanningEndEvent);
}