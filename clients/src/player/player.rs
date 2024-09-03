use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use std::collections::HashMap;
// use crate::playing_field::playing_field::Collision;
// use bevy::ecs::system::ParamSet;
use crate::{ Counter, GameState, PositionInitial };
use bevy_rapier3d::dynamics::{ LockedAxes, Velocity };
use bevy_rapier3d::prelude::{ Collider, GravityScale, RapierContext, RigidBody };
use bevy_renet::renet::{ DefaultChannel, RenetClient };
use bincode::serialize;
use store::{ GameEvent, Position };

use crate::playing_field::playing_field::{ check_player_collision, Collision };
// use bevy::sprite::collide_aabb::Collision;
// use bevy_rapier3d::prelude::RapierContext;

#[derive(Component)]
#[allow(dead_code)]
pub struct Player {
    pub id: i32,
    pub name: String,
    pub speed: f32,
    pub camera_offset: Vec3,
    pub size: Vec2,
    pub lives: u8,
}
#[derive(Component)]
#[allow(dead_code)]
pub struct PlayerCamera;

#[derive(Component)]
#[allow(dead_code)]
pub struct Weapon;
impl Player {
    pub fn new(id: i32, name: String, speed: f32, size: Vec2, lives: u8) -> Self {
        Player {
            id,
            name,
            speed,
            camera_offset: Vec3::new(0.0, 0.4, 0.8),
            size,
            lives,
        }
    }
    #[allow(dead_code)]
    pub fn player_lives(&self) -> u8 {
        self.lives
    }
}

#[allow(dead_code)]
pub fn move_player(
    mut client: ResMut<RenetClient>,
    mut query: Query<(Entity, &Player, &mut Transform, &mut Velocity)>,
    keyboard: Res<Input<KeyCode>>,
    mut mouse_motion: EventReader<MouseMotion>,
    windows: Query<&Window>,
    rapier_context: Res<RapierContext>,
    collider_query: Query<Entity, (With<Collision>, Without<Player>)>,
    location: ResMut<PositionInitial>,
    mut counter: ResMut<Counter>,
    game_state: Res<GameState>
) {
    let window = windows.single();
    if window.cursor.grab_mode == bevy::window::CursorGrabMode::None {
        return;
    }
    if !game_state.has_ended {
        let mut mouse_delta = Vec2::ZERO;
        for ev in mouse_motion.read() {
            mouse_delta += ev.delta;
        }
        for (entity, player, mut transform, _velocity) in query.iter_mut() {
            let a = counter.val;
            if a < 1 {
                transform.translation = Vec3::new(location.x, location.y, location.z);
            }
            counter.val += 1;

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

            if
                !check_player_collision(
                    entity,
                    &transform,
                    movement,
                    &rapier_context,
                    &collider_query
                ) &&
                game_state.has_started
            {
                transform.translation += movement;
            }

            // Assurez-vous que le joueur reste au sol
            transform.translation.y = 0.2;

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
                            player_list: HashMap::new(),
                            vision: (mouse_delta.x, mouse_delta.y),
                        })
                    ).unwrap()
                );
            }
        }
    }
}

#[allow(dead_code)]
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
#[allow(dead_code)]
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
            Player::new(player_id as i32, "Player".to_string(), 5.0, Vec2::new(0.5, 0.5), 3),
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

    // Spawn the camera and attach it to the weapon
    commands
        .spawn((
            Camera3dBundle {
                transform: Transform::from_xyz(-0.25, 0.7, 0.0), // Adjust camera position relative to a weapon
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
