use bevy::prelude::*;
use std::default::Default;
// use bevy::sprite::collide_aabb::collide;
use crate::player::player::Player;
use bevy_rapier3d::dynamics::RigidBody;
use bevy_rapier3d::prelude::Collider;
use crate::player_2d::player_2d::MinimapElement;

use bevy_rapier3d::prelude::*; // version bevy_rapier3d = "0.17.0"

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
        let arena_size = 28.0;

        let wall_height = 5.0;
        let wall_thickness = 0.5;

        let mut spawn_wall = |commands: &mut Commands, position: Vec3, size: Vec3| {
            commands
                .spawn(PbrBundle {
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
                .insert(Collision::Wall {
                    size: Vec2::new(size.x, size.z),
                });
        };

        // North Wall
        spawn_wall(
            &mut commands,
            Vec3::new(0.0, wall_height / 2.0, -arena_size / 2.0),
            Vec3::new(arena_size, wall_height, wall_thickness),
        );

        // South Wall
        spawn_wall(
            &mut commands,
            Vec3::new(0.0, wall_height / 2.0, arena_size / 2.0),
            Vec3::new(arena_size, wall_height, wall_thickness),
        );

        // East Wall
        spawn_wall(
            &mut commands,
            Vec3::new(arena_size / 2.0, wall_height / 2.0, 0.0),
            Vec3::new(wall_thickness, wall_height, arena_size),
        );

        // West Wall
        spawn_wall(
            &mut commands,
            Vec3::new(-arena_size / 2.0, wall_height / 2.0, 0.0),
            Vec3::new(wall_thickness, wall_height, arena_size),
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
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.3 })),
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
        commands
            .spawn(PbrBundle {
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
                lives: 3,
            });
    }
}

pub fn create_maze(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    name: &str,
) {
    let maze = get_mazes(name);
    let wall_height = 2.0;
    let wall_thickness = 0.5;
    let cell_size = 2.0;

    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            let x = col_index as f32 * cell_size - 9.0;
            let z = row_index as f32 * cell_size - 9.0;

            match cell {
                4 => {
                    // Vers le bas et la droite
                    spawn_wall(commands, meshes, materials, Vec3::new(x, wall_height / 2.0, z + cell_size / 2.0), Vec3::new(wall_thickness, wall_height, cell_size));
                    spawn_wall(commands, meshes, materials, Vec3::new(x + cell_size / 2.0, wall_height / 2.0, z), Vec3::new(cell_size, wall_height, wall_thickness));
                    spawn_minimap_wall(commands, x, z, true, true)
                }
                3 => {
                    // Vers la droite
                    spawn_wall(commands, meshes, materials, Vec3::new(x + cell_size / 2.0, wall_height / 2.0, z), Vec3::new(cell_size, wall_height, wall_thickness));
                    spawn_minimap_wall(commands, x, z, false, true)
                }
                1 => {
                    // Vers le bas
                    spawn_wall(commands, meshes, materials, Vec3::new(x, wall_height / 2.0, z + cell_size / 2.0), Vec3::new(wall_thickness, wall_height, cell_size));
                    spawn_minimap_wall(commands, x, z, true, false)
                }
                2 => {
                    // Fin de ligne (pas de mur)
                }
                _ => {}
            }
        }
    }
}

fn spawn_wall(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    size: Vec3,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(size.x, size.y, size.z))),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.8, 0.8, 0.8),
                ..Default::default()
            }),
            transform: Transform::from_translation(position),
            ..Default::default()
        },
        RigidBody::Fixed,
        Collider::cuboid(size.x * 0.5, size.y * 0.5, size.z * 0.5),
    ));
}



fn spawn_minimap_wall(
    commands: &mut Commands,
    x: f32,
    z: f32,
    vertical: bool,
    horizontal: bool,
) {
    let minimap_x = (x + 14.0) * (180.0 / 28.0);
    let minimap_z = (z + 14.0) * (180.0 / 28.0);
    if vertical {
        commands.spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(minimap_x + 10.0),
                    top: Val::Px(minimap_z + 10.0),
                    width: Val::Px(2.0),
                    height: Val::Px(14.0), // (180/28) * 2
                    ..default()
                },
                background_color: Color::GREEN.into(),
                ..default()
            },
            MinimapElement,
        ));
    }
    if horizontal {
        commands.spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(minimap_x + 10.0),
                    top: Val::Px(minimap_z + 10.0),
                    width: Val::Px(14.0), // (180/28) * 2
                    height: Val::Px(2.0),
                    ..default()
                },
                background_color: Color::GREEN.into(),
                ..default()
            },
            MinimapElement,
        ));
    }
}


fn get_mazes(name: &str) -> Vec<Vec<u8>> {
    if name == "Map1" {
        return  vec![
            vec![4, 3, 2, 3, 3, 3, 3, 4, 3, 3, 1],
            vec![1, 1, 1, 3, 3, 2, 1, 1, 3, 1, 1],
            vec![1, 2, 2, 2, 3, 3, 2, 2, 2, 3, 1],
            vec![1, 3, 3, 2, 3, 3, 1, 2, 3, 2, 1],
            vec![1, 1, 3, 2, 3, 2, 3, 2, 3, 1, 2],
            vec![2, 3, 3, 2, 1, 2, 3, 3, 2, 1, 1],
            vec![1, 3, 3, 2, 1, 2, 2, 1, 2, 1, 1],
            vec![1, 2, 3, 1, 1, 2, 3, 2, 1, 2, 1],
            vec![1, 3, 2, 2, 3, 2, 1, 2, 1, 2, 1],
            vec![1, 3, 3, 3, 2, 4, 2, 2, 2, 3, 1],
            vec![3, 3, 3, 3, 3, 3, 3, 3, 2, 3, 2]
        ]
    } else if name == "Map2" {
        return vec![
            vec![4, 3, 2, 3, 3, 4, 3, 4, 3, 3, 1],
            vec![1, 1, 1, 3, 3, 2, 1, 1, 3, 1, 1],
            vec![1, 2, 2, 3, 3, 3, 2, 1, 2, 3, 1],
            vec![1, 3, 3, 2, 3, 3, 1, 2, 3, 2, 1],
            vec![1, 1, 3, 2, 3, 2, 3, 2, 3, 1, 1],
            vec![1, 3, 3, 2, 1, 2, 3, 3, 2, 1, 1],
            vec![1, 3, 3, 2, 1, 2, 2, 1, 1, 1, 1],
            vec![1, 2, 3, 1, 4, 2, 3, 2, 4, 2, 1],
            vec![1, 3, 2, 2, 3, 2, 1, 2, 4, 2, 1],
            vec![1, 3, 3, 3, 2, 4, 2, 2, 2, 3, 1],
            vec![3, 3, 3, 3, 3, 3, 3, 3, 2, 3, 2]
        ];
    }else {
        vec![
            vec![4, 3, 3, 3, 3, 4, 3, 4, 3, 3, 1],
            vec![1, 1, 1, 3, 3, 2, 1, 1, 3, 1, 1],
            vec![1, 2, 4, 3, 4, 3, 2, 1, 3, 3, 1],
            vec![1, 3, 3, 2, 3, 3, 1, 2, 3, 3, 1],
            vec![1, 1, 3, 2, 4, 2, 3, 2, 3, 1, 1],
            vec![1, 3, 3, 2, 4, 3, 3, 3, 2, 1, 1],
            vec![1, 3, 3, 2, 1, 4, 2, 4, 1, 1, 1],
            vec![1, 4, 3, 1, 4, 2, 3, 2, 4, 2, 1],
            vec![1, 3, 2, 4, 3, 2, 1, 4, 4, 2, 1],
            vec![1, 3, 3, 3, 2, 4, 2, 2, 2, 3, 1],
            vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 2]
        ]
    }
}

// pub fn handle_collisions(
//     mut player_query: Query<(Entity, &mut Transform, &Collider), With<Player>>,
//     collider_query: Query<(Entity, &Collider), With<Collision>>,
//     rapier_context: Res<RapierContext>,
// ) {
//     if let Ok((player_entity, mut player_transform, player_collider)) = player_query.get_single_mut() {
//         for (wall_entity, wall_collider) in collider_query.iter() {
//             if let Some(contact_pair) = rapier_context.contact_pair(player_entity, wall_entity) {
//                 if contact_pair.has_any_active_contacts() {
//                     for manifold in contact_pair.manifolds() {
//                         let normal = manifold.normal();
//                         for contact_point in manifold.points() {
//                             let depth = contact_point.dist();
//                             if depth < 0.0 {
//                                 // Ajuster la position du joueur pour éviter la pénétration
//                                 player_transform.translation += Vec3::new(normal.x, normal.y, normal.z) * depth.abs();
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }

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

// pub fn handle_collisions(
//     mut player_query: Query<(&mut Transform, &Player)>,
//     collider_query: Query<(&Transform, &Collision), Without<Player>>,
// ) {
//     if let Ok((mut player_transform, player)) = player_query.get_single_mut() {
//         let player_size = Vec3::new(player.size.x, 1.0, player.size.y);
//
//         for (transform, collision) in collider_query.iter() {
//             let collider_size = match collision {
//                 Collision::Wall { size } => Vec3::new(size.x, 3.0, size.y),
//                 Collision::Ground => Vec3::new(22.0, 0.1, 22.0),
//             };
//
//             if let Some(collision) = collide(
//                 player_transform.translation,
//                 Vec2::new(player_size.x, player_size.z), // Taille du joueur en 2D
//                 transform.translation,
//                 Vec2::new(collider_size.x, collider_size.z), // Taille du collider en 2D
//             ) {
//                 match collision {
//                     bevy::sprite::collide_aabb::Collision::Left => {
//                         println!("Collision Left");
//                         player_transform.translation.x = transform.translation.x - (collider_size.x + player_size.x) * 0.5;
//                     }
//                     bevy::sprite::collide_aabb::Collision::Right => {
//                         println!("Collision Right");
//                         player_transform.translation.x = transform.translation.x + (collider_size.x + player_size.x) * 0.5;
//                         // player_transform.translation = player_transform.translation;
//                     }
//                     bevy::sprite::collide_aabb::Collision::Top => {
//                         println!("Collision Top");
//                         player_transform.translation.z = transform.translation.z - (collider_size.z + player_size.z) * 0.5;
//                     }
//                     bevy::sprite::collide_aabb::Collision::Bottom => {
//                         println!("Collision Bottom");
//                         player_transform.translation.z = transform.translation.z + (collider_size.z + player_size.z) * 0.5;
//                     }
//                     bevy::sprite::collide_aabb::Collision::Inside => {
//                         println!("Collision Inside");
//                         // Réduire la vitesse du joueur à zéro pour éviter de traverser le mur
//                         // Vous pouvez ajuster cette logique pour reculer légèrement le joueur
//                         player_transform.translation = player_transform.translation;
//                     }
//                 }
//             }
//         }
//     }
// }
