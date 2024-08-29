use crate::{player::player::Player, playing_field::playing_field::Collision};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy::window::Window;
use bevy::input::mouse::MouseButton;

#[derive(Component)]
pub struct Projectile {
    pub speed: f32,
}

#[derive(Component)]
pub struct Laser {
    pub max_distance: f32,
    pub lifetime: Timer,
}

#[derive(Bundle)]
pub struct LaserBundle {
    laser: Laser,
    pbr_bundle: PbrBundle,
}

pub fn fire_laser(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mouse: Res<Input<MouseButton>>,
    query: Query<(&Transform, &Player)>,
    windows: Query<&Window>,
        camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        if let Ok((player_transform, _player)) = query.get_single() {
            if let Ok((camera, camera_transform)) = camera_query.get_single() {
                let window = windows.single();                
                if let Some(ray) = ray_from_screenspace(
                    &window,
                    &camera,
                    &camera_transform,
                    Vec2::new(window.width() / 2.0, window.height() / 2.0),
                ) {
                    let spawn_point = player_transform.translation + player_transform.forward() * 3.5;
                    let laser_direction = ray.direction;
                    let laser_length = 3.0;
                    let laser_width = 0.09;

                    commands.spawn(LaserBundle {
                        laser: Laser {
                            max_distance: laser_length,
                            lifetime: Timer::from_seconds(0.1, TimerMode::Once),
                        },
                        pbr_bundle: PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Box::new(
                                laser_width,
                                laser_width,
                                laser_length,
                            ))),
                            material: materials.add(StandardMaterial {
                                base_color: Color::RED,
                                emissive: Color::rgba_linear(1.0, 0.0, 0.0, 1.0),
                                ..default()
                            }),
                            transform: Transform::from_translation(spawn_point)
                                .looking_to(laser_direction, Vec3::Y)
                                .with_scale(Vec3::new(0.07, 0.09, -1.0)),
                            ..default()
                        },
                    });
                }
            }
        }
    }
}


pub fn update_lasers(
    mut commands: Commands,
    mut laser_query: Query<(Entity, &mut Laser, &mut Transform)>,
    time: Res<Time>,
    rapier_context: Res<RapierContext>,
) {
    for (entity, mut laser, mut transform) in laser_query.iter_mut() {
        laser.lifetime.tick(time.delta());

        // Si la durée de vie du
        if laser.lifetime.finished() {
            commands.entity(entity).despawn();
            continue;
        }

        let ray_origin = transform.translation;
        let ray_direction = transform.forward();

        if let Some((_, intersection)) = rapier_context.cast_ray(
            ray_origin,
            ray_direction,
            laser.max_distance,
            true,
            QueryFilter::default(),
        ) {
            let hit_distance = intersection.tanh();
            transform.scale.z = hit_distance;
        } else {
            transform.scale.z = laser.max_distance;
        }
    }
}
fn ray_from_screenspace(
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
    cursor_position: Vec2,
) -> Option<Ray> {
    camera.viewport_to_world(camera_transform, cursor_position)
}
// impl Projectile {
// pub fn fire_projectile(
//     mut commands: Commands,
//     keyboard: Res<Input<KeyCode>>,
//     query: Query<(&Transform, &Player, &Velocity)>,
//     asset_server: Res<AssetServer>,
// ) {
//     if keyboard.just_pressed(KeyCode::Space) {
//         if let Ok((transform, player, player_velocity)) = query.get_single() {
//             let forward = transform.forward();
//             let spawn_point = transform.translation + forward * 1.0 + Vec3::new(0.0, 0.1, 0.0);
//
//             let projectile_handle: Handle<Scene> = asset_server.load("projectile/bullet.glb#Scene0");
//
//             let projectile_velocity = player_velocity.linvel + forward * 20.0;
//
//             commands.spawn((
//                 SceneBundle {
//                     scene: projectile_handle,
//                     transform: Transform::from_translation(spawn_point)
//                         .looking_to(forward, Vec3::Y)
//                         .with_scale(Vec3::splat(0.1)),
//                     ..default()
//                 },
//                 Projectile { speed: 5.0 },
//                 RigidBody::Dynamic,
//                 Collider::ball(0.1),
//                 Velocity::linear(projectile_velocity),
//             ));
//         }
//     }
// }

// pub fn update_projectiles(
//     mut commands: Commands,
//     mut query: Query<(Entity, &mut Transform, &Projectile)>,
//     time: Res<Time>,
// ) {
//     for (entity, mut transform, projectile) in query.iter_mut() {
//         let forward = transform.forward();
//         transform.translation += forward * projectile.speed * time.delta_seconds();
//
//         if transform.translation.length() > 100.0 {
//             commands.entity(entity).despawn();
//         }
//     }
// }

// }
pub fn handle_projectile_collisions(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Transform), With<Projectile>>,
    rapier_context: Res<RapierContext>,
    collider_query: Query<Entity, (With<Collision>, Without<Projectile>)>,
) {
    for (projectile_entity, projectile_transform) in projectile_query.iter() {
        if check_projectile_collision(
            projectile_entity,
            projectile_transform,
            &rapier_context,
            &collider_query,
        ) {
            commands.entity(projectile_entity).despawn();
        }
    }
}

fn check_projectile_collision(
    projectile_entity: Entity,
    projectile_transform: &Transform,
    rapier_context: &RapierContext,
    collider_query: &Query<Entity, (With<Collision>, Without<Projectile>)>,
) -> bool {
    let ray_origin = projectile_transform.translation;
    let ray_direction = projectile_transform.forward();
    let max_toi = 0.1; // Distance courte pour vérifier juste devant le projectile

    if let Some((hit_entity, _hit_position)) = rapier_context.cast_ray(
        ray_origin,
        ray_direction,
        max_toi,
        true,
        QueryFilter::default().exclude_collider(projectile_entity),
    ) {
        // Vérifier si l'entité touchée fait partie des colliders valides
        collider_query.get(hit_entity).is_ok()
    } else {
        false
    }
}
