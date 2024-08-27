use std::u8;

use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
// use crate::playing_field::playing_field::Collision;
// use bevy::ecs::system::ParamSet;
use bevy_rapier3d::dynamics::{ LockedAxes, Velocity };
use bevy_rapier3d::prelude::{ Collider, GravityScale, RapierContext, RigidBody };
use bevy_renet::renet::{ DefaultChannel, RenetClient };
use bincode::serialize;
use store::{ GameEvent, Position };

use crate::playing_field::playing_field::{ check_player_collision, Collision };
// use bevy::sprite::collide_aabb::Collision;
// use bevy_rapier3d::prelude::RapierContext;

#[derive(Component)]
pub struct Player {
    #[allow(dead_code)]
    pub id: i32,
    #[allow(dead_code)]
    pub name: String,
    pub speed: f32,
    pub camera_offset: Vec3,
    pub size: Vec2,
}
#[derive(Component)]
pub struct PlayerCamera;
#[derive(Component)]
pub struct Weapon;
impl Player {
    pub fn new(id: i32, name: String, speed: f32, size: Vec2) -> Self {
        Player {
            id,
            name,
            speed,
            camera_offset: Vec3::new(0.0, 0.2, 0.8),
            size,
        }
    }
}

pub fn move_player(
    mut client: ResMut<RenetClient>,
    mut query: Query<(Entity, &Player, &mut Transform, &mut Velocity)>,
    keyboard: Res<Input<KeyCode>>,
    mut mouse_motion: EventReader<MouseMotion>,
    windows: Query<&Window>,
    rapier_context: Res<RapierContext>,
    collider_query: Query<Entity, (With<Collision>, Without<Player>)>
) {
    let window = windows.single();
    if window.cursor.grab_mode == bevy::window::CursorGrabMode::None {
        return;
    }

    let mut mouse_delta = Vec2::ZERO;
    for ev in mouse_motion.read() {
        mouse_delta += ev.delta;
    }

    for (entity, player, mut transform, mut velocity) in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        if keyboard.pressed(KeyCode::W) {
            direction += transform.forward();
        }
        if keyboard.pressed(KeyCode::S) {
            direction += transform.back();
        }
        if keyboard.pressed(KeyCode::A) {
            direction += transform.left();
        }
        if keyboard.pressed(KeyCode::D) {
            direction += transform.right();
        }

        direction = direction.normalize_or_zero();

        let movement = direction * player.speed * 0.016;

        // Vérifier la collision avant de déplacer le joueur

        // Rotation du joueur (et de l'arme)
        transform.rotate_y(-mouse_delta.x * 0.002);
        // // Rotation verticale (limitée)
        // let mut camera_transform = transform.clone();
        // camera_transform.rotate_local_x(-mouse_delta.y * 0.002);
        // //
        // // // Limiter la rotation verticale
        // let up = camera_transform.up();
        // if up.y > 0.999 && up.y < 0.995 {
        //     *transform = camera_transform;
        //
        // }
        if !check_player_collision(entity, &transform, movement, &rapier_context, &collider_query) {
            transform.translation += movement;
        }

        // Rotation verticale (limitée)
        // let max_vertical_angle = 0.4 ; // Limite de l'angle de rotation verticale (en radians)
        // let rotation_x = -mouse_delta.y * 0.002 ;
        // let new_x_rotation = transform.rotation.to_euler(EulerRot::YXZ).0 + rotation_x ;
        //
        // if new_x_rotation.abs() <= max_vertical_angle {
        //     transform.rotate_local_x(rotation_x);
        // }
        //
        // // Empêcher tout mouvement vertical involontaire
        // Assurez-vous que le joueur reste au sol
        transform.translation.y = 0.2;
        // println!(
        //     "Player position: x={:?}, y={:?}, z={:?}",
        //     transform.translation.x,
        //     transform.translation.y,
        //     transform.translation.z
        // );
        if client.is_connected() {
            client.send_message(
                DefaultChannel::ReliableOrdered,
                serialize(
                    &(GameEvent::PlayerMove {
                        at: Position::new(
                            transform.translation.x,
                            transform.translation.y,
                            transform.translation.z
                        ),
                        player_id: u8::MAX,
                    })
                ).unwrap()
            );
        }
    }
}

pub fn grab_mouse(
    mut windows: Query<&mut Window>,
    mouse: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>
) {
    let mut window = windows.single_mut();
    if mouse.just_pressed(MouseButton::Left) {
        window.cursor.grab_mode = bevy::window::CursorGrabMode::Confined;
        window.cursor.visible = false;
    }
    if key.just_pressed(KeyCode::Escape) {
        window.cursor.grab_mode = bevy::window::CursorGrabMode::None;
        window.cursor.visible = true;
    }
}
pub fn setup_player_and_camera(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    player_id: u8,
    x: f32,
    y: f32,
    z: f32
) {
    // Spawn the player
    let player_handle: Handle<Scene> = asset_server.load("armes/arme1.glb#Scene0");
    // let player_handle:Handle<Scene> = asset_server.load("armes/Soldier.glb#Scene0");
    let player_entity = commands
        .spawn((
            Player::new(player_id as i32, "Player".to_string(), 5.0, Vec2::new(0.5, 0.5)),
            SceneBundle {
                scene: player_handle,
                transform: Transform::from_xyz(x, y, z).with_scale(Vec3::splat(0.4)),
                ..default()
            },
            // Controls manuel du joueur sans se soucier d'influence externe
            RigidBody::KinematicPositionBased,
            Collider::ball(0.5),
            Velocity::default(), // Assurez-vous que cette ligne est présente
            LockedAxes::ROTATION_LOCKED,
            GravityScale(0.0),
        ))
        .id();
    // Spawn the weapon and attach it to the player
    // let weapon_handle: Handle<Scene> = asset_server.load("armes/arme1.glb#Scene0");
    // let weapon_entity = commands.spawn((
    //     Weapon,
    //     SceneBundle {
    //         scene: weapon_handle,
    //         transform: Transform::from_xyz(0.0, 0.2, 2.5).with_scale(Vec3::splat(0.3)),
    //         ..default()
    //     },
    // )).id();
    // commands.entity(player_entity).add_child(player_entity);

    // Spawn the camera and attach it to the weapon
    commands
        .spawn((
            Camera3dBundle {
                transform: Transform::from_xyz(0.0, 0.8, 0.0), // Adjust camera position relative to a weapon
                ..default()
            },
        ))
        .set_parent(player_entity);

    commands
        .spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(-1.0, 2.0, -4.0),
            ..default()
        })
        .set_parent(player_entity);
}
