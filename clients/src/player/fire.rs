use crate::enemys::enemys::Enemy;
use crate::GameState;
use crate::{ player::player::Player, playing_field::playing_field::Collision };
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy::window::Window;
use bevy::input::mouse::MouseButton;
use bevy_renet::renet::{ DefaultChannel, RenetClient };
use bincode::serialize;
use store::GameEvent;

#[derive(Component)]
#[allow(dead_code)]
pub struct Projectile {
    pub speed: f32,
    pub lifetime: Timer,
}

#[derive(Bundle)]
pub struct ProjectileBundle {
    projectile: Projectile,
    pbr_bundle: PbrBundle,
    rigid_body: RigidBody,
    collider: Collider,
    velocity: Velocity,
}

#[allow(dead_code)]
pub fn fire_projectile(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mouse: Res<Input<MouseButton>>,
    query: Query<(&Transform, &Player)>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>
) {
    if mouse.just_pressed(MouseButton::Left) {
        if let Ok((player_transform, _player)) = query.get_single() {
            if let Ok((camera, camera_transform)) = camera_query.get_single() {
                let window = windows.single();
                let center = Vec2::new(window.width() / 2.0, window.height() / 2.0 - 20.0);

                if let Some(ray) = ray_from_screenspace(&camera, camera_transform, center) {
                    let spawn_point =
                        player_transform.translation + player_transform.forward() * 0.6;
                    let projectile_direction = ray.direction;

                    commands.spawn(ProjectileBundle {
                        projectile: Projectile {
                            speed: 100.0,
                            lifetime: Timer::from_seconds(5.0, TimerMode::Once),
                        },
                        pbr_bundle: PbrBundle {
                            mesh: meshes.add(
                                Mesh::try_from(shape::Icosphere {
                                    radius: 0.01,
                                    subdivisions: 1,
                                }).unwrap()
                            ),
                            material: materials.add(StandardMaterial {
                                base_color: Color::ORANGE_RED,
                                emissive: Color::rgba_linear(1.0, 0.0, 0.0, 1.0),
                                ..default()
                            }),
                            transform: Transform::from_translation(spawn_point),
                            ..default()
                        },
                        rigid_body: RigidBody::KinematicVelocityBased,
                        collider: Collider::ball(0.03),
                        velocity: Velocity::linear(projectile_direction * 50.0),
                    });
                }
            }
        }
    }
}

#[allow(dead_code)]
pub fn update_projectiles(
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &mut Projectile, &Velocity)>,
    time: Res<Time>,
    rapier_context: Res<RapierContext>
) {
    for (entity, mut projectile, velocity) in projectile_query.iter_mut() {
        projectile.lifetime.tick(time.delta());

        if projectile.lifetime.finished() {
            commands.entity(entity).despawn();
            continue;
        }

        let ray_origin = velocity.linvel.normalize() * 0.05;
        let ray_direction = velocity.linvel.normalize();

        if
            rapier_context
                .cast_ray(ray_origin, ray_direction, 0.1, true, QueryFilter::default())
                .is_some()
        {
            commands.entity(entity).despawn();
        }
    }
}

#[allow(dead_code)]
fn ray_from_screenspace(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    cursor_position: Vec2
) -> Option<Ray> {
    camera.viewport_to_world(camera_transform, cursor_position)
}

#[allow(dead_code)]
pub fn handle_projectile_collisions(
    mut client: ResMut<RenetClient>,
    game_state: Res<GameState>,
    mut commands: Commands,
    projectile_query: Query<(Entity, &Transform), With<Projectile>>,
    mut enemy_query: Query<(Entity, &mut Enemy, &Transform)>
) {
    const IMPACT_DISTANCE: f32 = 0.45;

    for (projectile_entity, projectile_transform) in projectile_query.iter() {
        let projectile_position = projectile_transform.translation;

        for (enemy_entity, mut enemy, enemy_transform) in enemy_query.iter_mut() {
            let enemy_position = enemy_transform.translation;
            let distance = projectile_position.distance(enemy_position);

            if distance < IMPACT_DISTANCE {
                // Réduire les vies de l'ennemi
                enemy.lives = enemy.lives.saturating_sub(1);
                if client.is_connected() && !game_state.has_ended {
                    let impact_event = GameEvent::Impact { id: enemy.id };
                    client.send_message(
                        DefaultChannel::ReliableOrdered,
                        serialize(&impact_event).unwrap()
                    );
                    println!("  💥:::::::::Enemy hit! Lives remaining: {}:::::::::💥", enemy.lives);
                    if enemy.lives == 0 {
                        commands.entity(enemy_entity).despawn();
                    }
                    // Despawn le projectile
                    commands.entity(projectile_entity).despawn();
                }

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
    collider_query: &Query<Entity, (With<Collider>, Without<Projectile>)>
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
        }
    );

    hit_entity.map(|entity| (entity, hit_toi))
}

#[allow(dead_code)]
pub fn check_player_collision(
    player_entity: Entity,
    weapon_transform: &Transform,
    direction: Vec3,
    rapier_context: &RapierContext,
    _collider_query: &Query<Entity, (With<Collision>, Without<Player>)>
) -> bool {
    // Position future du joueur
    let _future_position = weapon_transform.translation + direction;

    // Lancer un rayon pour détecter une collision
    let ray_origin = weapon_transform.translation;
    let ray_direction = direction.normalize();

    let max_toi = direction.length(); // Distance maximale du rayon

    if
        let Some((_hit_entity, _hit_position)) = rapier_context.cast_ray(
            ray_origin,
            ray_direction,
            max_toi + 1.5,
            true,
            QueryFilter::default().exclude_collider(player_entity)
        )
    {
        // Si un objet est détecté sur la trajectoire, il y a une collision
        return true;
    }

    false // Pas de collision détectée
}
