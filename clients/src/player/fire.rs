use crate::enemys::enemys::Enemy;
use crate::{player::player::Player, playing_field::playing_field::Collision};
use bevy::input::mouse::MouseButton;
use bevy::prelude::*;
use bevy::window::Window;
use bevy_rapier3d::prelude::*;

// #[derive(Component)]
#[allow(dead_code)]
// pub struct Projectile {
//     pub speed: f32,
//     pub lifetime: Timer,
// }

#[derive(Bundle)]
pub struct ProjectileBundle {
    projectile: Projectile,
    pbr_bundle: PbrBundle,
    rigid_body: RigidBody,
    collider: Collider,
    velocity: Velocity,
}
#[derive(Component)]
pub struct Projectile {
    pub speed: f32,
    pub lifetime: Timer,
    pub initial_velocity: Vec3,
    pub gravity: f32,
}

pub fn fire_projectile(
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
                let center = Vec2::new(window.width() / 2.0, window.height() / 2.0 - 20.);

                if let Some(ray) = ray_from_screenspace(&camera, camera_transform, center) {
                    let spawn_point = player_transform.translation + player_transform.forward() * 0.60;
                    let projectile_direction = ray.direction;

                    let initial_speed = 50.0; // Ajustez cette valeur selon vos besoins
                    let initial_velocity = projectile_direction * initial_speed;

                    commands.spawn((
                        Projectile {
                            speed: initial_speed,
                            lifetime: Timer::from_seconds(5.0, TimerMode::Once),
                            initial_velocity,
                            gravity: 9.81, // Ajustez cette valeur selon vos besoins
                        },
                        PbrBundle {
                            mesh: meshes.add(Mesh::try_from(shape::Icosphere {
                                radius: 0.01,
                                subdivisions: 1,
                            }).unwrap()),
                            material: materials.add(StandardMaterial {
                                base_color: Color::ORANGE_RED,
                                emissive: Color::rgba_linear(1.0, 0.0, 0.0, 1.0),
                                ..default()
                            }),
                            transform: Transform::from_translation(spawn_point),
                            ..default()
                        },
                        RigidBody::Dynamic,
                        Collider::ball(0.03),
                        Velocity::linear(initial_velocity),
                    ));
                }
            }
        }
    }
}

pub fn update_projectiles(
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &mut Projectile, &mut Transform, &mut Velocity)>,
    time: Res<Time>,
) {
    for (entity, mut projectile, mut transform, mut velocity) in projectile_query.iter_mut() {
        projectile.lifetime.tick(time.delta());

        if projectile.lifetime.finished() {
            commands.entity(entity).despawn();
            continue;
        }

        // Mise à jour de la vélocité avec la gravité
        velocity.linvel += Vec3::new(0.0, -projectile.gravity, 0.0) * time.delta_seconds();

        // Mise à jour de la position
        transform.translation += velocity.linvel * time.delta_seconds();

        // Rotation du projectile pour qu'il pointe dans la direction du mouvement
        if velocity.linvel.length_squared() > 0.01 {
            transform.look_to(velocity.linvel, Vec3::Y);
        }
    }
}

#[allow(dead_code)]
fn ray_from_screenspace(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    cursor_position: Vec2,
) -> Option<Ray> {
    camera.viewport_to_world(camera_transform, cursor_position)
}

#[allow(dead_code)]
pub fn handle_projectile_collisions(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Transform), With<Projectile>>,
    mut enemy_query: Query<(Entity, &mut Enemy, &Transform)>,
) {
    const IMPACT_DISTANCE: f32 = 0.2; // Ajustez cette valeur selon vos besoins

    for (projectile_entity, projectile_transform) in projectile_query.iter() {
        let projectile_position = projectile_transform.translation;

        for (enemy_entity, mut enemy, enemy_transform) in enemy_query.iter_mut() {
            let enemy_position = enemy_transform.translation;
            let distance = projectile_position.distance(enemy_position);

            if distance < IMPACT_DISTANCE {
              
                // println!("Position de la balle : {:?}", projectile_position);
                // println!("Position de l'ennemi : {:?}", enemy_position);
                // println!("Distance : {}", distance);

                // Réduire les vies de l'ennemi
                enemy.lives = enemy.lives.saturating_sub(1);
                println!("  💥:::::::::Enemy hit! Lives remaining: {}:::::::::💥", enemy.lives);

                // Despawn le projectile
                commands.entity(projectile_entity).despawn();

                // Optionnel : Despawn l'ennemi s'il n'a plus de vies
                // if enemy.lives == 0 {
                //     commands.entity(enemy_entity).despawn();
                //     println!("Ennemi éliminé !");
                // }

                break;
            }
        }
    }
}


#[allow(dead_code)]
pub fn check_projectile_collision(
    projectile_entity: Entity,
    projectile_transform: &Transform,
    rapier_context: &RapierContext,
    direction: Vec3,
    collider_query: &Query<Entity, (With<Collider>, Without<Projectile>)>,
) -> Option<(Entity, f32)> {
    let ray_origin = projectile_transform.translation;
    let ray_direction = direction.normalize();
    let max_toi = direction.length(); // Maximum ray distance plus a small buffer
    let mut hit_entity = None;
    let mut hit_toi = f32::MAX;

    rapier_context.intersections_with_ray(
        ray_origin,
        ray_direction,
        max_toi,
        true,
        QueryFilter::default().exclude_collider(projectile_entity),
        |entity, intersection| {
            // Check if the intersected entity is in the collider_query
            if collider_query.get(entity).is_ok() {
                hit_entity = Some(entity);
                hit_toi = intersection.toi;
                false // Stop the ray cast when we find a valid collision
            } else {
                true // Continue the ray cast if it's not a valid collider
            }
        },
    );

    hit_entity.map(|entity| (entity, hit_toi))
}

#[allow(dead_code)]
pub fn check_player_collision(
    player_entity: Entity,
    weapon_transform: &Transform,
    direction: Vec3,
    rapier_context: &RapierContext,
    _collider_query: &Query<Entity, (With<Collision>, Without<Player>)>,
) -> bool {
    // Position future du joueur
    let _future_position = weapon_transform.translation + direction;

    // Lancer un rayon pour détecter une collision
    let ray_origin = weapon_transform.translation;
    let ray_direction = direction.normalize();

    let max_toi = direction.length(); // Distance maximale du rayon

    if let Some((_hit_entity, _hit_position)) = rapier_context.cast_ray(
        ray_origin,
        ray_direction,
        max_toi + 1.5,
        true,
        QueryFilter::default().exclude_collider(player_entity),
    ) {
        // Si un objet est détecté sur la trajectoire, il y a une collision
        return true;
    }

    false // Pas de collision détectée
}
