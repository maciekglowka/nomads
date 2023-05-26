use bevy::prelude::*;

use super::super::UiAssets;
use super::super::events::MenuCloseEvent;

const FONT_SIZE: f32 = 24.;

#[derive(Component)]
pub struct SelectionMenu<T: Send + Sync + 'static> {
    pub options: Vec<SelectionMenuOption<T>>,
    pub index: usize
}
impl<T: Send + Sync + 'static> SelectionMenu<T> {
    pub fn new(options: Vec<SelectionMenuOption<T>>) -> Self {
        SelectionMenu { options, index: 0 }
    }
    pub fn get_current(&mut self) -> &mut SelectionMenuOption<T> {
        &mut self.options[self.index]
    }
}

pub struct SelectionMenuOption<T: Send + Sync> {
    pub label: String,
    pub value: Option<T>
}
impl<T: Send + Sync> SelectionMenuOption<T> {
    pub fn new(label: String, value: T) -> Self {
        SelectionMenuOption { label, value: Some(value) }
    }
}

pub fn close_menu<T: Send + Sync + 'static> (
    keys: Res<Input<KeyCode>>,
    mut ev_close: EventWriter<MenuCloseEvent>
) {
    if keys.just_pressed(KeyCode::Escape) { 
        ev_close.send(MenuCloseEvent(false));
    };
    if keys.just_pressed(KeyCode::Space) {
        ev_close.send(MenuCloseEvent(true));
    };
}

pub fn update_menu<T: Send + Sync + 'static> (
    mut commands: Commands,
    mut query: Query<(Entity, &mut SelectionMenu<T>)>,
    keys: Res<Input<KeyCode>>,
    assets: Res<UiAssets>
) {
    let mut dir = None;
    if keys.just_pressed(KeyCode::W) {
        dir = Some(-1);
    }
    if keys.just_pressed(KeyCode::S) {
        dir = Some(1);
    }
    let Some(dir) = dir else { return };
    let Ok((entity, mut menu)) = query.get_single_mut() else { return };
    commands.entity(entity).despawn_descendants();
    menu.index = (menu.index as isize + dir)
        .max(0)
        .min(menu.options.len() as isize - 1) as usize;
    draw_options(&mut commands, entity, menu.as_ref(), assets.as_ref());
}

pub fn draw_menu<T: Send + Sync + 'static> (
    commands: &mut Commands,
    options: Vec<SelectionMenuOption<T>>,
    assets: &UiAssets
) {
    let menu = SelectionMenu::<T>::new(options);
    let entity = commands.spawn(
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    margin: UiRect::all(Val::Auto),
                    padding: UiRect::all(Val::Px(4.)),
                    ..Default::default()
                },
                background_color: Color::DARK_GRAY.into(),
                ..Default::default()
            }
        )
        .id();
    draw_options(commands, entity, &menu, assets);
    commands.entity(entity).insert(menu);
}

fn draw_options<T: Send + Sync + 'static> (
    commands: &mut Commands,
    parent: Entity,
    menu: &SelectionMenu<T>,
    assets: &UiAssets
) {
    for (idx, option) in menu.options.iter().enumerate() {
        let entity = commands.spawn(
                TextBundle {
                    text: Text::from_section(
                        option.label.clone(),
                        TextStyle { 
                            font: assets.font.clone(),
                            font_size: FONT_SIZE,
                            color: if idx==menu.index { Color::WHITE } else { Color::SILVER }
                        }
                    ),
                    ..Default::default()
                }
            )
            .id();
        commands.entity(parent).add_child(entity);
    }
}