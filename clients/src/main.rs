use crate::player::player::Player;
use bevy::prelude::*;
use bevy_rapier3d::plugin::{NoUserData, RapierPhysicsPlugin};
use bevy_renet::{transport::NetcodeClientPlugin, RenetClientPlugin};
use multiplayer_fps::{get_input, handle_connection, PlayerSpawnInfo, setup_networking};
use std::net::SocketAddr;

// use bevy::sprite::collide_aabb::collide;
// use bevy::render::debug::DebugLines;
// use bevy_gltf::Gltf;
mod player;
mod player_2d;
mod playing_field;

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
        eprintln!(
            "❌ Username is too long (max {} characters)",
            MAX_USERNAME_LENGTH
        );
        return;
    }

    let (client, transport) = setup_networking(&server_addr, &username);

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "IBG".into(),
                resolution: (1500.0, 1000.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            RenetClientPlugin,
            NetcodeClientPlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
        ))
        .add_systems(
            Startup,
            (
                //player::player::setup_player_and_camera,
                playing_field::playing_field::Fields::spawn_ground,
                player_2d::player_2d::setup_minimap,
                // playing_field::playing_field::Fields::spawn_object,
                // playing_field::playing_field::Fields::spawn_player,
                setup,
            ),
        )
        .insert_resource(client)
        .insert_resource(transport)
        // .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
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
            )
                .chain(),
        )
        .run();
}
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
   // Charger le modèle
    // let scene_handle: Handle<Scene> = asset_server.load("mages/mage1_2.glb#Scene0");
    // // Spawner le modèle
    // commands.spawn((
    //     SceneBundle {
    //         scene: scene_handle,
    //         transform: Transform::from_xyz(-5.0, -2.3, -5.0).with_scale(Vec3::splat(0.8)),
    //         ..default()
    //     },
    //     GltfWall,
    // ));
    
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
        0.0, 0.0, 0.0, // Position par défaut
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
// fn handle_gltf_wall_collisions(
//     mut player_query: Query<(&mut Transform, &Player)>,
//     wall_query: Query<&GlobalTransform, With<GltfWall>>,
// ) {
//     if let Ok((mut player_transform, player)) = player_query.get_single_mut() {
//         for wall_transform in wall_query.iter() {
//                 let wall_scale = wall_transform.compute_transform().scale;
//                 let wall_size = Vec3::new(2.0, 2.0, 2.0) * wall_scale.x;
//                 let wall_pos = wall_transform.translation();
//                 let collision = collide(
//                     player_transform.translation,
//                     Vec2::new(player.size.x, player.size.y),
//                     wall_pos,
//                     Vec2::new(wall_size.x, wall_size.z),
//                 );
//                 if let Some(collision) = collision {
//                     match collision {
//                         bevy::sprite::collide_aabb::Collision::Left => {
//                             println!("Main Collision Left");
//                             player_transform.translation.x = wall_pos.x - (wall_size.x + player.size.x) * 0.5;
//                         }
//                         bevy::sprite::collide_aabb::Collision::Right => {
//                             println!("Main Collision Right");
//                             player_transform.translation.x = wall_pos.x + (wall_size.x + player.size.x) * 0.5;
//                             // player_transform.translation.x = transform.translation.x + (collider_size.x + player_size.x) * 0.5;
//                         }
//                         bevy::sprite::collide_aabb::Collision::Top => {
//                             println!("Main Collision Top");
//                             player_transform.translation.z = wall_pos.z - (wall_size.z + player.size.y) * 0.5;
//                         }
//                         bevy::sprite::collide_aabb::Collision::Bottom => {
//                             println!("Main Collision Bottom");
//                             player_transform.translation.z = wall_pos.z + (wall_size.z + player.size.y) * 0.5;
//                         }
//                         bevy::sprite::collide_aabb::Collision::Inside => {
//                             println!("Main Collision Inside");
//                             // Gérez le cas où le joueur est à l'intérieur du mur
//                         }
//                     }
//                     println!("Player pos: {:?}, Wall pos: {:?}", player_transform.translation, wall_pos);
//                 }
//             }
//         }
//     }
fn update_minimap(
    player_query: Query<&Transform, With<Player>>,
    mut minimap_query: Query<&mut Transform, With<MinimapPlayer>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok(mut minimap_transform) = minimap_query.get_single_mut() {
            // Met à jour la position du joueur sur la minimap
            minimap_transform.translation = Vec3::new(
                200.0 + player_transform.translation.x * 10.0, // Ajuste l'échelle pour la minimap
                200.0 + player_transform.translation.z * 10.0,
                0.0,
            );
        }
    }
}


// pub fn crate_mage(
//     name : &str,
//     mut commands : Commands,
//     mut mesh : ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>
// ) {
//     let mage = mages(name);
//     for v in &mage {
//         for k in v {
//            if *k == 4 {
//                println!("Vers le bas et la droit");
//            } else if *k == 3 {
//                println!("Vers la droite");
//            } else if *k == 2 {
//                println!("Fin de ligne")
//            } else {
//                println!("Vers le bas");
//            }
//         }
//     }
//     println!("{:?}", mage);
// }
// pub fn mages(name: &str) -> Vec<Vec<u8>> {
//     if name == "Map1" {
//         return  vec![
//             vec![4, 4, 3, 3, 2, 4, 3, 4, 3, 3, 1],
//             vec![1, 1, 1, 3, 3, 2, 1, 1, 3, 1, 1],
//             vec![1, 2, 4, 3, 3, 3, 2, 1, 3, 1, 1],
//             vec![4, 3, 3, 2, 3, 3, 1, 2, 1, 3, 1],
//             vec![1, 4, 3, 2, 4, 2, 4, 3, 3, 1, 1],
//             vec![4, 2, 4, 3, 4, 3, 2, 4, 2, 1, 1],
//             vec![1, 3, 2, 1, 1, 4, 3, 3, 1, 1, 1],
//             vec![1, 3, 4, 1, 1, 1, 4, 2, 1, 2, 1],
//             vec![1, 1, 2, 1, 4, 2, 1, 1, 4, 3, 1],
//             vec![1, 3, 3, 1, 2, 4, 2, 1, 2, 1, 1],
//             vec![3, 3, 3, 3, 3, 2, 3, 3, 3, 3, 2]
//         ];
//     } else if name == "Map2" {
//         return  vec![
//             vec![],
//         ];
//     }
//     vec![
//         vec![],
//     ]
// }

// fn debug_draw_system(
//     mut gizmos: Gizmos,
//     wall_query: Query<&GlobalTransform, With<GltfWall>>,
// ) {
//     for wall_transform in wall_query.iter() {
//         let wall_pos = wall_transform.translation();
//         let wall_scale = wall_transform.scale();
//         let wall_size = Vec3::new(2.0, 2.0, 2.0) * wall_scale.x;
//
//         gizmos.cuboid(
//             Transform::from_translation(wall_pos).with_scale(wall_size),
//             Color::RED,
//         );
//     }
// }
// fn debug_draw_system(
//     mut lines: ResMut<DebugLines>,
//     player_query: Query<&Transform, With<Player>>,
//     wall_query: Query<&GlobalTransform, With<GltfWall>>,
// ) {
//     let player_transform = player_query.single();
//     for wall_transform in wall_query.iter() {
//         let wall_pos = wall_transform.translation();
//         let wall_size = Vec3::new(2.0, 3.0, 2.0);
//
//         lines.line_colored(
//             wall_pos - wall_size * 0.5,
//             wall_pos + wall_size * 0.5,
//             0.0,
//             Color::RED,
//         );
//     }
//
//     lines.line_colored(
//         player_transform.translation,
//         player_transform.translation + Vec3::Y,
//         0.0,
//         Color::GREEN,
//     );
// }
//
// fn setup(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     // Assuming you have a GLTF plugin or loader set up
//     let map_handle: Handle<Gltf> = asset_server.load("mages/mage3D.glb");
//
//     commands.spawn(PointLightBundle {
//         transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
//         ..default()
//     });
//     commands.spawn(PointLightBundle {
//         point_light: PointLight {
//             intensity: 1500.0,
//             shadows_enabled: true,
//             ..default()
//         },
//         transform: Transform::from_xyz(4.0, 8.0, 4.0),
//         ..default()
//     });
//
//     // Load the GLTF model and spawn it with necessary components
//     commands.spawn(SceneBundle {
//         scene: asset_server.load("mages/mage3D.glb"),
//         ..default()
//     });
//
// }
// #[derive(Component)]
// struct Ground;
//
// fn spawn_plancher(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     // plane
//     commands.spawn((
//         PbrBundle {
//             mesh: meshes.add(Plane3d::default().mesh().size(20., 20.)),
//             material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
//             ..default()
//         },
//         Ground,
//     ));
//
//     // light
//     commands.spawn(DirectionalLightBundle {
//         transform: Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
//         ..default()
//     });
//
//     // camera
//     commands.spawn(Camera3dBundle {
//         transform: Transform::from_xyz(0.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
//         ..default()
//     });
// }
//
// pub fn spawn_object(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ){
//     // cube
//     commands.spawn(PbrBundle {
//         mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
//         material: materials.add(Color::srgb_u8(124, 144, 255)),
//         transform: Transform::from_xyz(0.0, 0.5, 0.0),
//         ..default()
//
//     });
// }
//
//
// pub fn spawn_player(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     commands.spawn((
//         PbrBundle {
//             // mesh: meshes.add(Mesh::try_from(Capsule::default()).unwrap()),
//             material: materials.add(Color::from(SILVER)),
//             transform: Transform::from_xyz(0.0, 1.0, 0.0),
//             ..default()
//         },
//         Player,
//     ));
// }
// #[derive(Component)]
// struct Player;
// pub fn spawn_camera(mut commands: Commands) {
//     let camera = Camera3dBundle {
//         transform: Transform::from_xyz(0.0, 2.5, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
//         ..default()
//     };
//     commands.spawn(camera);
// }
// pub fn spawn_lumiere(mut commands: Commands) {
//     let lumiere = PointLightBundle {
//        point_light: PointLight {
//            intensity: 10000.0,
//            shadows_enabled: true,
//            ..default()
//        } ,
//         transform: Transform::from_xyz(4.0, 8.0, 4.0),
//         ..default()
//     };
//     commands.spawn(lumiere);
// }
// fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
//     commands.spawn(Camera2dBundle::default());
//
//     commands.spawn(TextBundle::from_section(
//         "Bienvenue dans IBG!",
//         TextStyle {
//             font: asset_server.load("fonts/EduAUVICWANTHand-VariableFont_wght.ttf"),
//             font_size: 40.0,
//             color: Color::WHITE,
//         },
//     ));
// }
