use bevy::{
    asset::AssetServer,
    diagnostic::{ DiagnosticsStore, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin },
    prelude::{ Color, Commands, Component, Query, Res, ResMut, TextBundle, With },
    text::{ Text, TextSection, TextStyle },
    ui::{ Display, PositionType, Style, Val },
    utils::default,
};

use crate::GameState;

#[derive(Component)]
pub struct FpsText;
pub fn fps_display_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
    mut query_style: Query<&mut Style, With<FpsText>>,
    game_state: Res<GameState>
) {
    if game_state.has_started {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                for mut text in query.iter_mut() {
                    text.sections[0].value = format!("{:.2} FPS", average);
                }
                for mut style in query_style.iter_mut() {
                    style.display = Display::DEFAULT;
                }
            }
        }
    }
}

pub fn setupfps(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle {
            text: Text::from_sections([
                TextSection::new("0 FPS", TextStyle {
                    font: asset_server.load("fonts/8-bit-hud.ttf"),
                    font_size: 25.0,
                    color: Color::RED,
                }),
            ]),
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                right: Val::Px(10.0),
                display: Display::None,
                ..default()
            },
            ..default()
        },
        FpsText,
    ));
}
