use bevy::prelude::*;

use crate::player::Player;

use super::super::{FONT_SIZE, MENU_PADDING, UiAssets, clear};

const HEIGHT: f32 = 40.;

#[derive(Component)]
pub struct StatusBar;

pub fn draw(
    mut commands: Commands,
    assets: Res<UiAssets>,
    player: Res<Player>,
    status_query: Query<Entity, With<StatusBar>>
) {
    if !player.is_changed() { return };
    clear::<StatusBar>(&mut commands, &status_query);
    let container = commands.spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect { left: Val::Px(0.), top: Val::Px(0.), ..Default::default() },
                    size: Size::new(Val::Percent(100.), Val::Px(HEIGHT)),
                    padding: UiRect::all(MENU_PADDING),
                    ..Default::default()
                },
                background_color: Color::DARK_GRAY.into(),
                ..Default::default()
            },
            StatusBar
        ))
        .id();

    // TODO add more stats
    let info = commands.spawn(
            TextBundle {
                text: Text::from_section(
                    format!("Goods: {:?}", player.goods),
                    TextStyle { 
                        font: assets.font.clone(),
                        font_size: FONT_SIZE,
                        color: Color::WHITE
                    }
                ),
                ..Default::default()
            }
        )
        .id();
    commands.entity(container).add_child(info);
}