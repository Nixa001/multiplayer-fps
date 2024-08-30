use crate::player::player::Player;
use bevy::prelude::*;
use bevy_rapier3d::plugin::{ NoUserData, RapierPhysicsPlugin };
use bevy_renet::{ transport::NetcodeClientPlugin, RenetClientPlugin };
use games::{ gamestate::{ setup_timer, display_timer }, fps::*, live::* };
use multiplayer_fps::{
    get_input,
    handle_connection,
    setup_networking,
    Counter,
    EnnemyCreated,
    GameState,
    GameTimer,
    ListPlayer,
    PlayerSpawnInfo,
    PositionInitial,
};
use std::{ i32, net::SocketAddr };
use bevy::diagnostic::{ FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin };
// use bevy::sprite::collide_aabb::collide;
// use bevy::render::debug::DebugLines;
// use bevy_gltf::Gltf;
mod player;
mod player_2d;
mod playing_field;
mod enemys;

mod games;
// use bevy::diagnostic::{ FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
// use bevy::render::debug::DebugLines;
// use bevy_gltf::Gltf;
// use crate::games::fps::*;
// #[derive(Component)]
// struct GltfWall;

#[derive(Component)]
struct MinimapPlayer;

const MAX_USERNAME_LENGTH: usize = 248;
fn main() {
    let server_ip = get_input("Enter server IP address: ");
    let server_addr: SocketAddr = match server_ip.parse() {
        Ok(addr) => addr,
        Err(_) => {
            eprintln!("❌ Invalid IP address!");
            return;
        }
    };

    let username = get_input("Enter your username: ");
    if username.is_empty() {
        eprintln!("❌ Please provide a username");
        return;
    }

    if username.len() > MAX_USERNAME_LENGTH {
        eprintln!("❌ Username is too long (max {} characters)", MAX_USERNAME_LENGTH);
        return;
    }

    let (client, transport) = setup_networking(&server_addr, &username);
    let position = PositionInitial::default();
    let counter = Counter::default();
    let list_user = ListPlayer::default();
    let timer = GameTimer { sec: i32::MAX };
    let game_state = GameState::new();
    let ennemy_created = EnnemyCreated { val: false };
    App::new()
        .insert_resource(client)
        .insert_resource(transport)
        .insert_resource(position)
        .insert_resource(counter)
        .insert_resource(list_user)
        .insert_resource(timer)
        .insert_resource(game_state)
        .insert_resource(ennemy_created)
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "IBG".into(),
                    resolution: (1500.0, 1000.0).into(),
                    resizable: true,
                    ..default()
                }),
                ..default()
            })
        )
        .add_plugins((
            RenetClientPlugin,
            NetcodeClientPlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
        ))
        .add_systems(Startup, (
            //player::player::setup_player_and_camera,
            playing_field::playing_field::Fields::spawn_ground,
            player_2d::player_2d::setup_minimap,
            // playing_field::playing_field::Fields::spawn_object,
            // playing_field::playing_field::Fields::spawn_player,
            setup,
            setup_timer,
            setupfps,
            setuplives,
        ))
        // .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                fps_display_system,
                display_lives,
                display_timer,
                handle_connection,
                player::player::move_player,
                player::player::grab_mouse,
                player::fire::fire_laser,
                player::fire::update_lasers,
                player::fire::handle_projectile_collisions,
                player_2d::player_2d::update_minimap,
                // playing_field::playing_field::handle_collisions,
                // handle_gltf_wall_collisions,
                // debug_draw_system,
                enemys::enemys::update_enemys_position,
            ).chain()
        )
        .run();
}
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Caméra
    // commands.spawn(Camera3dBundle {
    //    transform: Transform::from_xyz(10.0, 45.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    //    ..default()
    // });
    // Lumière
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 10.0, 0.0),
        ..default()
    });
    // Joueur sur la minimap
    commands.spawn((
        MinimapPlayer,
        SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                ..default()
            },
            transform: Transform::from_scale(Vec3::splat(5.0)), // Échelle du joueur sur la minimap
            ..default()
        },
    ));

    commands.insert_resource(PlayerSpawnInfo {
        player_id: None,
        position: None,
    });

    // Créez le joueur avec une position par défaut
    player::player::setup_player_and_camera(
        &mut commands,
        &asset_server,
        0, // ID temporaire
        0.0,
        0.0,
        0.0
        // Position par défaut
    );
}
fn check_model_loaded(asset_server: Res<AssetServer>, scene_assets: Res<Assets<Scene>>) {
    let scene_handle: Handle<Scene> = asset_server.load("mages/mage1_3.glb#Scene0");
    if scene_assets.contains(&scene_handle) {
        println!("Le modèle GLTF a été chargé avec succès!");
    } else {
        println!("Le modèle GLTF n'est pas encore chargé...");
    }
}

fn update_minimap(
    player_query: Query<&Transform, With<Player>>,
    mut minimap_query: Query<&mut Transform, With<MinimapPlayer>>
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok(mut minimap_transform) = minimap_query.get_single_mut() {
            // Met à jour la position du joueur sur la minimap
            minimap_transform.translation = Vec3::new(
                200.0 + player_transform.translation.x * 10.0, // Ajuste l'échelle pour la minimap
                200.0 + player_transform.translation.z * 10.0,
                0.0
            );
        }
    }
}
