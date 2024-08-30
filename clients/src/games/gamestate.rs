use bevy::{
    asset::AssetServer,
    prelude::{ Color, Commands, Component, Query, Res, ResMut, Resource, TextBundle, With },
    text::{ Text, TextSection, TextStyle },
    ui::{ Display, PositionType, Style, Val },
    utils::default,
};

use crate::{ GameState, GameTimer };
#[derive(Component)]
pub struct TimerText;

pub fn display_timer(
    mut query_text: Query<&mut Text, With<TimerText>>,
    mut query_style: Query<&mut Style, With<TimerText>>,
    timer: ResMut<GameTimer>,
    game_state: Res<GameState>
) {
    if game_state.is_waiting && timer.sec != i32::MAX {
        for mut text in query_text.iter_mut() {
            let color = if timer.sec <= 10 { Color::RED } else { Color::OLIVE };
            text.sections[0].value = format!("Game starts in {}...s", timer.sec);
            text.sections[0].style.color = color;
        }
        for mut style in query_style.iter_mut() {
            style.display = Display::DEFAULT;
        }
    } else if game_state.has_started {
        for mut style in query_style.iter_mut() {
            style.display = Display::None;
        }
    }
}

pub fn setup_timer(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle {
            text: Text::from_sections([
                TextSection::new(format!("Game starts in 20s"), TextStyle {
                    font: asset_server.load("fonts/8-bit-hud.ttf"),
                    font_size: 25.0,
                    color: Color::BLACK,
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
        TimerText,
    ));
}
