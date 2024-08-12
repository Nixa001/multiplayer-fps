use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use crate::playing_field::playing_field::Collision;
use bevy::ecs::system::ParamSet;

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
    mut query_set: ParamSet<(
        Query<(&Player, &mut Transform)>,
        Query<&mut Transform, With<PlayerCamera>>,
    )>,
    keyboard: Res<Input<KeyCode>>,
    mut mouse_motion: EventReader<MouseMotion>,
    time: Res<Time>,
    windows: Query<&Window>,
) {
    let window = windows.single();
    if window.cursor.grab_mode == bevy::window::CursorGrabMode::None {
        return;
    }

    let mut mouse_delta = Vec2::ZERO;
    for ev in mouse_motion.read() {
        mouse_delta += ev.delta;
    }

    // Mouvement du joueur
    if let Ok((player, mut transform)) = query_set.p0().get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard.pressed(KeyCode::W) { direction += transform.forward(); }
        if keyboard.pressed(KeyCode::S) { direction += transform.back(); }
        if keyboard.pressed(KeyCode::A) { direction += transform.left(); }
        if keyboard.pressed(KeyCode::D) { direction += transform.right(); }

        if direction != Vec3::ZERO {
            direction = direction.normalize();
            transform.translation += direction * player.speed * time.delta_seconds();
        }

        // Rotation du joueur (et de l'arme)
        transform.rotate_y(-mouse_delta.x * 0.002);
    }

    // Rotation de la caméra (seulement verticalement)
    if let Ok(mut camera_transform) = query_set.p1().get_single_mut() {
        camera_transform.rotate_local_x(-mouse_delta.y * 0.002);
        camera_transform.rotation.x = camera_transform.rotation.x.clamp(-1.0, 1.0);
    }
}
pub fn grab_mouse(
    mut windows: Query<&mut Window>,
    mouse: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let mut window = windows.single_mut();

    if mouse.just_pressed(MouseButton::Left) {
        window.cursor.visible = false;
        window.cursor.grab_mode = bevy::window::CursorGrabMode::Confined;
    }

    if key.just_pressed(KeyCode::Escape) {
        window.cursor.visible = true;
        window.cursor.grab_mode = bevy::window::CursorGrabMode::None;
    }
}

pub fn setup_player_and_camera(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Spawn the player
    let player_handle: Handle<Scene> = asset_server.load("armes/arme1.glb#Scene0");
    // let player_handle:Handle<Scene> = asset_server.load("armes/Soldier.glb#Scene0");

    let player_entity = commands.spawn((
        Player::new(1, "Player".to_string(), 3.0, Vec2::new(1.0, 1.0)), // Ajoutez la taille du joueur
        SceneBundle {
            scene: player_handle,
            transform: Transform::from_xyz(0.0, 0.2, 8.0).with_scale(Vec3::splat(0.1)),
            ..default()
        },
    )).id();

    // Spawn the weapon and attach it to the player
    let weapon_handle: Handle<Scene> = asset_server.load("armes/arme1.glb#Scene0"); // Replace with your weapon model
    let weapon_entity = commands.spawn((
        Weapon,
        SceneBundle {
            scene: weapon_handle,
            transform: Transform::from_xyz(0.0, 0.2, 0.5), // Adjust weapon position relative to player
            ..default()
        },
    )).id();

    commands.entity(player_entity).add_child(weapon_entity);

    // Spawn the camera and attach it to the weapon
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.8, 0.8), // Adjust camera position relative to weapon
            ..default()
        },
    )).set_parent(weapon_entity);
}
