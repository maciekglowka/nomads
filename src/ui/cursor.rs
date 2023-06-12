use bevy::prelude::*;

use crate::common::components::{Position, Worker};
use crate::graphics::{math::hex_to_v3, TILE_SIZE, OVERLAY_Z};
use crate::hex::Hex;
use crate::player::{
    commands::{PlaceWorker, RemoveWorker},
    Player
};

use super::{GameUiState, UiAssets};
use super::elements::select_menu::{SelectMenu, SelectMenuOption, draw_menu};
use super::events::MenuCloseEvent;

#[derive(Component)]
pub struct Cursor;

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
    next_state.set(GameUiState::WorkerPlaceMenu);
}

pub fn move_cursor(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Position, &mut Transform), With<Cursor>>
) {
    let mut dir = None;
    let Ok((mut position, mut transform)) = query.get_single_mut() else { return };

    if keys.just_pressed(KeyCode::W) {
        dir = Some(Hex::new(0, 1));
    }
    if keys.just_pressed(KeyCode::S) {
        dir = Some(Hex::new(0, -1));
    }
    if keys.just_pressed(KeyCode::A) {
        dir = Some(Hex::new(-1, 0));
    }
    if keys.just_pressed(KeyCode::D) {
        dir = Some(Hex::new(1, 0));
    }
    // if keys.just_pressed(KeyCode::A) {
    //     dir = match position.0.q % 2 {
    //         0 => Some(Hex::new(-1, 1)),
    //         _ => Some(Hex::new(-1, 0))
    //     };
    // }
    // if keys.just_pressed(KeyCode::D) {
    //     dir = match position.0.q % 2 {
    //         0 => Some(Hex::new(1, 0)),
    //         _ => Some(Hex::new(1, -1))
    //     };
    // }
    if let Some(dir) = dir {
        position.0 += dir;
        transform.translation = hex_to_v3(position.0, OVERLAY_Z); 
    }
}

pub fn spawn_cursor(
    mut commands: Commands,
    assets: Res<UiAssets>
) {
    let mut sprite = TextureAtlasSprite::new(0);
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
    let hex = Hex::default();
    let v = hex_to_v3(hex, OVERLAY_Z); 
    commands.spawn((
            Cursor,
            Position(hex),
            SpriteSheetBundle {
                sprite,
                texture_atlas: assets.cursor_texture.clone(),
                transform: Transform::from_translation(v),
                ..Default::default()
            }
        ));
}

// const DIR_KEY_MAPPING: [(KeyCode, Hex); 4] = [
//     (KeyCode::W, Hex::new(0, 1)), (KeyCode::S, Hex::new()),
//     (KeyCode::A, Vector2Int::LEFT), (KeyCode::D, Vector2Int::RIGHT),
// ];