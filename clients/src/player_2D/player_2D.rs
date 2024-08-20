use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::player::player::Player;

#[derive(Component)]
pub struct MinimapElement;

#[derive(Component)]
pub struct MinimapPlayer;

pub fn setup_minimap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    //minimap
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Px(10.0),
                top: Val::Px(10.0),
                width: Val::Px(200.0),
                height: Val::Px(200.0),

                ..default()
            },
            background_color: Color::rgba(0.1, 0.1, 0.1, 0.7).into(),
            ..default()
        },
        MinimapElement,
        ));

//     Spawn player
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                // top: Val::Px(12.0),
                // right: Val::Px(12.0),
                width: Val::Px(5.0),
                height: Val::Px(5.0),
                ..default()
            },
            background_color: Color::RED.into(),
            ..default()
        },
        MinimapElement,
        MinimapPlayer,
        ));
}

pub fn update_minimap(
    player_query: Query<&Transform, With<Player>>,
    mut minimap_query: Query<&mut Style, With<MinimapPlayer>>,
    window: Query<&Window, With<PrimaryWindow>>,
){
    let window = window.single();
    if let Ok (player_trnsform) = player_query.get_single(){
        if let Ok (mut minimap_style) = minimap_query.get_single_mut(){
            // Convertir les donnees 3D en 2D
            let minimap_x = (player_trnsform.translation.x + 14.0) * (180.0 / 28.0);
            let minimap_y = (player_trnsform.translation.z + 14.0) * (180.0 / 28.0);

            // Mettre a jour la position du joueur sur la minimap
            minimap_style.right = Val::Px(minimap_x + 10.0);
            minimap_style.top = Val::Px(minimap_y + 10.0);
        }
    }
}

