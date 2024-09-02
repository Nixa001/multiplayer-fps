use bevy::{
    asset::AssetServer,
    prelude::{Color, Commands, Component, Query, Res, TextBundle, With},
    text::{Text, TextSection, TextStyle},
    ui::{Display, PositionType, Style, Val},
};

use crate::GameState;

#[derive(Component)]
#[allow(dead_code)]
pub struct Lives;

#[allow(dead_code)]
pub fn display_lives(
    mut query: Query<&mut Text, With<Lives>>,
    mut query_style: Query<&mut Style, With<Lives>>,
    game_state: Res<GameState>,
) {
    if game_state.has_started {
        for mut text in query.iter_mut() {
            text.sections[0].value = format!("Lives: {}", 1);
        }
        for mut style in query_style.iter_mut() {
            style.display = Display::DEFAULT;
        }
    }
}

#[allow(dead_code)]
pub fn setuplives(mut commands: Commands, asset: Res<AssetServer>) {
    commands.spawn((
        TextBundle {
            text: Text::from_sections([TextSection::new(
                format!("Lives: {}", 3),
                TextStyle {
                    font: asset.load("fonts/8-bit-hud.ttf"),
                    font_size: 25.0,
                    color: Color::OLIVE,
                },
            )]),
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Px(10.0),
                top: Val::Px(50.0),
                display: Display::None,
                ..Default::default()
            },
            ..Default::default()
        },
        Lives,
    ));
}
