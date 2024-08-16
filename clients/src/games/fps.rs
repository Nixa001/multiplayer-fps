use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};


#[derive(Component)]
struct FpsText;
fn fps_display_system(diagnostics: Res<DiagnosticsStore>, mut query: Query<&mut Text, With<FpsText>>) {
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(average) = fps.average() {
            for mut text in query.iter_mut() {
                text.sections[0].value = format!("{:.2} FPS", average);
            }
        }
    }
}

fn setupfps(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle {
            text: Text::from_sections([
                TextSection::new(
                    "0 FPS",
                    TextStyle {
                        font: asset_server.load("fonts/Atop-R99O3.ttf"),
                        font_size: 20.0,
                        color: Color::BLACK,
                    },
                ),
            ]),
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                right: Val::Px(10.0),
                ..default()
            },
            ..default()
        },
        FpsText,
    ));
}