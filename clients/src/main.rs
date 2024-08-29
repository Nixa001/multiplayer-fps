use bevy::prelude::*;
use bevy_rapier3d::plugin::{ NoUserData, RapierPhysicsPlugin };
use bevy_renet::{ transport::NetcodeClientPlugin, RenetClientPlugin };
use multiplayer_fps::{Counter, get_input, handle_connection, PlayerSpawnInfo, PositionInitial, setup_networking};
use std::net::SocketAddr;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
// use bevy::sprite::collide_aabb::collide;
// use bevy::render::debug::DebugLines;
// use bevy_gltf::Gltf;
mod player;
mod player_2d;
mod playing_field;

mod games;
// use bevy::diagnostic::{ FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
// use bevy::render::debug::DebugLines;
// use bevy_gltf::Gltf;
// use crate::games::fps::*;
// #[derive(Component)]
// struct GltfWall;

#[derive(Component)]
struct MinimapPlayer;
#[derive(Component)]
struct Crosshair;
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
    App::new()
        .insert_resource(client)
        .insert_resource(transport)
        .insert_resource(position)
        .insert_resource(counter)
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
        .add_systems(Startup, setup_crosshair)
        .add_systems(Update, update_crosshair_position)
        // .add_systems(Update, (fire_projectile, update_projectiles))
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
            games::fps::setupfps,
        ))
        // .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                games::fps::fps_display_system,
                handle_connection,
                player::player::move_player,
                player::player::grab_mouse,
                player::fire::fire_projectile,
                player::fire::update_projectiles,
                // player::fire::handle_projectile_collisions,
                player_2d::player_2d::update_minimap,
                // playing_field::playing_field::handle_collisions,
                // handle_gltf_wall_collisions,
                // debug_draw_system,
            ).chain()
        )
        .run();
}
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>)
{


    // Caméra
    //commands.spawn(Camera3dBundle {
    //    transform: Transform::from_xyz(10.0, 45.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    //    ..default()
    //});
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


fn setup_crosshair(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Chargez l'image du viseur
    let crosshair_image = asset_server.load("viseur.png");
    
    commands.spawn((
        Crosshair,
        ImageBundle {
            image: UiImage::new(crosshair_image),
            style: Style {
                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        },
    ));
}

fn update_crosshair_position(
    mut crosshair_query: Query<&mut Style, With<Crosshair>>,
    windows: Query<&Window>,
) {
    let window = windows.single();
    if let Ok(mut style) = crosshair_query.get_single_mut() {
        style.left = Val::Px(window.width() / 2.0);
        style.top = Val::Px(window.height() / 2.0);
    }
}
