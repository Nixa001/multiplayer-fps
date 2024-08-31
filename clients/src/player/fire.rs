use crate::enemys::enemys::Enemy;
use crate::{player::player::Player, playing_field::playing_field::Collision};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy::window::Window;
use bevy::input::mouse::MouseButton;

#[derive(Component)]
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
                let center = Vec2::new(window.width() / 2.0, window.height() / 2.0 -20.);
                
                if let Some(ray) = ray_from_screenspace(
                    &window,
                    &camera,
                    camera_transform,
                    center,
                ) {
                    let spawn_point = player_transform.translation + player_transform.forward() * 0.60;
                    let projectile_direction = ray.direction;
                    
                    commands.spawn(ProjectileBundle {
                        projectile: Projectile { 
                            speed: 80.0,
                            lifetime: Timer::from_seconds(5.0, TimerMode::Once),
                        },
                        pbr_bundle: PbrBundle {
                            mesh: meshes.add(Mesh::try_from(shape::Icosphere { radius: 0.01, subdivisions: 1 }).unwrap()),
                            material: materials.add(StandardMaterial {
                                base_color: Color::ORANGE_RED,
                                emissive: Color::rgba_linear(0.0, 0.0, 0.0, 1.0),
                                ..default()
                            }),
                            transform: Transform::from_translation(spawn_point),
                            ..default()
                        },
                        rigid_body: RigidBody::KinematicVelocityBased,
                        collider: Collider::ball(0.05),
                        velocity: Velocity::linear(projectile_direction * 50.0),
                    });
                }
            }
        }
    }
}

pub fn update_projectiles(
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &mut Projectile, &Velocity)>,
    time: Res<Time>,
    rapier_context: Res<RapierContext>,
) {
    for (entity, mut projectile, velocity) in projectile_query.iter_mut() {
        projectile.lifetime.tick(time.delta());

        if projectile.lifetime.finished() {
            commands.entity(entity).despawn();
            continue;
        }

        let ray_origin = velocity.linvel.normalize() * 0.05;
        let ray_direction = velocity.linvel.normalize();
        
        if rapier_context.cast_ray(
            ray_origin,
            ray_direction,
            0.1,
            true,
            QueryFilter::default(),
        ).is_some() {
            commands.entity(entity).despawn();
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


pub fn handle_projectile_collisions(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Transform), With<Projectile>>,
    rapier_context: Res<RapierContext>,
    mut enemy_query: Query<(Entity, &mut Enemy), With<Collision>>,
) {
    for (projectile_entity, projectile_transform) in projectile_query.iter() {
        if let Some((hit_entity, hit_position)) = check_projectile_collision(
            projectile_entity,
            projectile_transform,
            &rapier_context,
        ) {
            // Check if the hit entity is an enemy
            if let Ok((_, mut enemy)) = enemy_query.get_mut(hit_entity) {
                // Decrement enemy lives
                enemy.lives = enemy.lives.saturating_sub(1);
                
                // If enemy has no lives left, despawn it
                if enemy.lives == 0 {
                    commands.entity(hit_entity).despawn();
                }
                
                // Print debug information
                println!("Enemy hit! Lives remaining: {}", enemy.lives);
            }
            
            // Despawn the projectile
            commands.entity(projectile_entity).despawn();
        }
    }
}

fn check_projectile_collision(
    projectile_entity: Entity,
    projectile_transform: &Transform,
    rapier_context: &RapierContext,
) -> Option<(Entity, f32)> {
    let ray_origin = projectile_transform.translation;
    let ray_direction = projectile_transform.forward();
    let max_toi = 0.1; // Short distance to check just in front of the projectile
    
    rapier_context.cast_ray(
        ray_origin,
        ray_direction,
        max_toi,
        true,
        QueryFilter::default().exclude_collider(projectile_entity),
    )
}