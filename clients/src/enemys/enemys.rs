use crate::ListPlayer;
use crate::{EnnemyCreated, GameState};
use bevy::prelude::*;
use bevy_rapier3d::dynamics::Velocity;
use bevy_rapier3d::prelude::{Collider, RigidBody};
use store::Position;

#[allow(dead_code)]
#[derive(Component)]
pub struct Enemy {
    pub id: u8,
    pub name: String,
    pub position: Position,
    pub lives: u8,
}

impl Enemy {
    pub fn new(id: u8, name: String, position: Position) -> Self {
        Enemy {
            id,
            name,
            position,
            lives: 3,
        }
    }
}

#[allow(dead_code)]
pub fn create_enemys(
    commands: &mut Commands,
    list_player: &ListPlayer,
    asset_server: &AssetServer,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    println!("------------Enemys-------{:?}", list_player.list);
    for (&id, player) in list_player.list.iter() {
        let enemy_mesh = meshes.add(Mesh::from(shape::Cylinder {
            radius: 0.15,
            height: 1.3,
            ..default()
        }));

        let enemy_material = materials.add(StandardMaterial {
            base_color: Color::rgba(1.0, 0.0, 0.0, 0.5), // Rouge semi-transparent
            alpha_mode: AlphaMode::Blend,
            ..default()
        });

        let enemy_model = asset_server.load("soldier/soldier2.glb#Scene0");

        let player_entity = commands
            .spawn((
                Enemy::new(id, format!("Enemy_{}", id), player.position.clone()),
                SpatialBundle {
                    transform: Transform::from_xyz(
                        player.position.x,
                        player.position.y,
                        player.position.z,
                    ),
                    ..default()
                },
                RigidBody::KinematicPositionBased,
                Collider::cylinder(1.3, 0.15),
                Velocity::default(),
            ))
            .insert(Name::new(format!("Enemy_{}", id)))
            .with_children(|parent| {
                // Spawn the transparent cylinder
                parent.spawn(PbrBundle {
                    mesh: enemy_mesh,
                    material: enemy_material,
                    ..default()
                });

                // Spawn the enemy model with a scale applied
                parent.spawn(SceneBundle {
                    scene: enemy_model,
                    transform: Transform {
                        translation: Vec3::new(0.0, 0.2, 0.0),
                        scale: Vec3::splat(0.02), // Apply the scale here
                        ..default()
                    },
                    ..default()
                });
            })
            .id();

        println!("Spawned enemy with ID: {:?}", player_entity);
    }
}

#[allow(dead_code)]
pub fn update_enemys_position(
    mut query: Query<(&mut Transform, &mut Enemy)>,
    mut commands: Commands,
    list_player: Res<ListPlayer>,
    asset_server: Res<AssetServer>,
    game_state: Res<GameState>,
    mut ennemy_created: ResMut<EnnemyCreated>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if game_state.has_started && ennemy_created.val {
        println!("❌❌❌❌");
        create_enemys(
            &mut commands,
            &list_player,
            &asset_server,
            &mut meshes,
            &mut materials,
        );
        ennemy_created.val = false;
    }
    for (mut transform, mut enemy) in query.iter_mut() {
        if let Some(player) = list_player.list.get(&enemy.id) {
            enemy.position = player.position.clone();
            transform.translation =
                Vec3::new(enemy.position.x, enemy.position.y - 0.2, enemy.position.z);
            transform.rotate_local_y(-player.vision.0 * 0.002);
        }
    }
}
