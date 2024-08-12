use bevy::prelude::*;
use std::default::Default;
use bevy::sprite::collide_aabb::collide;
use bevy_rapier3d::prelude::*; // version bevy_rapier3d = "0.17.0"
use crate::player::player::Player;
use bevy_rapier3d::dynamics::RigidBody;
use bevy_rapier3d::prelude::Collider;




#[derive(Bundle)]
struct CustomBundle {
    pbr: PbrBundle,
    collision: Collision,
    rigid_body: RigidBody,
    collider: Collider,
}


#[derive(Component)]
pub enum Collision {
    Wall { size: Vec2 },
    Ground,
}

impl Default for Collision {
    fn default() -> Self {
        Collision::Ground
    }
}

pub struct Fields;

impl Fields {
    pub fn spawn_ground(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        // Plane
        let arena_size = 22.0;
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: arena_size, subdivisions: 0 })),
            material: materials.add(StandardMaterial {
                base_color: Color::hex("7F7F7F").unwrap(),
                ..Default::default()
            }),
            ..Default::default()
        })
            .insert(RigidBody::Fixed)
            .insert(Collider::cuboid(arena_size * 0.5, 0.1, arena_size * 0.5))
            .insert(Collision::Ground);

        let wall_height = 5.0;
        let wall_thickness = 0.5;

        let mut spawn_wall = |commands: &mut Commands, position: Vec3, size: Vec3| {
            commands.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::new(size.x, size.y, size.z))),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgb(0.8, 0.8, 0.8),
                    ..Default::default()
                }),
                transform: Transform::from_translation(position),
                ..Default::default()
            })
                .insert(RigidBody::Fixed)
                .insert(Collider::cuboid(size.x * 0.5, size.y * 0.5, size.z * 0.5))
                .insert(Collision::Wall { size: Vec2::new(size.x, size.z) });
        };

        // North Wall
        spawn_wall(
            &mut commands,
            Vec3::new(0.0, wall_height / 2.0, -arena_size / 2.0),
            Vec3::new(arena_size, wall_height, wall_thickness)
        );

        // South Wall
        spawn_wall(
            &mut commands,
            Vec3::new(0.0, wall_height / 2.0, arena_size / 2.0),
            Vec3::new(arena_size, wall_height, wall_thickness)
        );

        // East Wall
        spawn_wall(
            &mut commands,
            Vec3::new(arena_size / 2.0, wall_height / 2.0, 0.0),
            Vec3::new(wall_thickness, wall_height, arena_size)
        );

        // West Wall
        spawn_wall(
            &mut commands,
            Vec3::new(-arena_size / 2.0, wall_height / 2.0, 0.0),
            Vec3::new(wall_thickness, wall_height, arena_size)
        );

        // Light
        commands.spawn(DirectionalLightBundle {
            transform: Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        });
    }

    pub fn spawn_object(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        // Cube
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb_u8(124, 144, 255),
                ..Default::default()
            }),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..Default::default()
        });
    }

    pub fn spawn_player(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        // Capsule
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule {
                radius: 0.5,
                rings: 4,
                depth: 1.0,
                latitudes: 8,
                longitudes: 16,
                uv_profile: shape::CapsuleUvProfile::Fixed,
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::SILVER,
                ..Default::default()
            }),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..Default::default()
        })
            .insert(Player {
                id: 1,
                name: "Player".to_string(),
                speed: 3.0,
                camera_offset: Vec3::new(0.0, 0.2, 0.8),
                size: Vec2::new(1.0, 1.0),
            });
    }
}


pub fn handle_collisions(
    mut player_query: Query<(&mut Transform, &Player)>,
    collider_query: Query<(&Transform, &Collision), Without<Player>>,
) {
    if let Ok((mut player_transform, player)) = player_query.get_single_mut() {
        // let player_pos = player_transform.translation;
        let _player_pos = player_transform.translation;
        let player_size = Vec3::new(player.size.x, 1.0, player.size.y);

        for (transform, collision) in collider_query.iter() {
            let collider_size = match collision {
                Collision::Wall { size } => Vec3::new(size.x, 3.0, size.y),
                Collision::Ground => Vec3::new(22.0, 0.1, 22.0),
            };

            let collision = collide(
                player_transform.translation,
                Vec2::new(player_size.x, player_size.z), // Convert to Vec2 for collide
                transform.translation,
                Vec2::new(collider_size.x, collider_size.z), // Convert to Vec2 for collide
            );

            if let Some(collision) = collision {
                match collision {
                    bevy::sprite::collide_aabb::Collision::Left => {
                        println!("Collision Left");
                        player_transform.translation.x = transform.translation.x - (collider_size.x + player_size.x) * 0.5;
                    }
                    bevy::sprite::collide_aabb::Collision::Right => {
                        println!("Collision Right");
                        player_transform.translation.x = transform.translation.x + (collider_size.x + player_size.x) * 0.5;
                    }
                    bevy::sprite::collide_aabb::Collision::Top => {
                        println!("Collision Top");
                        player_transform.translation.z = transform.translation.z - (collider_size.z + player_size.z) * 0.5;
                    }
                    bevy::sprite::collide_aabb::Collision::Bottom => {
                        println!("Collision Bottom");
                        player_transform.translation.z = transform.translation.z + (collider_size.z + player_size.z) * 0.5;
                    }
                    bevy::sprite::collide_aabb::Collision::Inside => {
                        println!("Collision Inside");
                        // let direction = player_transform.translation - transform.translation;
                        // player_transform.translation += direction.normalize() * 0.1;
                    }
                }
            }
        }
    }
}
