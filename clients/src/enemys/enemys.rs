use bevy::prelude::*;
use store::Position;
use crate::{ Counter, EnnemyCreated, GameState };
use crate::ListPlayer;
use crate::player::player::Player;
use bevy_rapier3d::dynamics::{ LockedAxes, Velocity };
use bevy_rapier3d::prelude::{ Collider, GravityScale, RapierContext, RigidBody };

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
            lives: 3
        }
    }
}

pub fn create_enemys(
    commands: &mut Commands,
    list_player: &ListPlayer,
    asset_server: &AssetServer,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>
) {
    println!("------------Enemys-------{:?}", list_player.list);
    for (&id, player) in list_player.list.iter() {
        let player_entity = commands.spawn((
            Enemy::new(id, format!("Enemy_{}", id), player.position.clone()),
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cylinder {
                    radius: 0.5,
                    height: 2.0,
                    ..default()
                })),
                material: materials.add(Color::RED.into()),
                transform: Transform::from_xyz(
                    player.position.x,
                    player.position.y,
                    player.position.z
                ),
                ..default()
            },
            RigidBody::KinematicPositionBased,
            Collider::cylinder(1.0, 0.5),
            Velocity::default(),
        ))
        .insert(Name::new(format!("Enemy_{}", id)))
        .id();
        println!("Spawned enemy with ID: {:?}", player_entity);
    }
}

pub fn update_enemys_position(
    mut query: Query<(&mut Transform, &mut Enemy)>,
    mut commands: Commands,
    list_player: Res<ListPlayer>,
    asset_server: Res<AssetServer>,
    game_state: Res<GameState>,
    mut ennemy_created: ResMut<EnnemyCreated>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    if game_state.has_started && ennemy_created.val {
        println!("❌❌❌❌");
        create_enemys(&mut commands, &list_player, &asset_server, &mut meshes, &mut materials);
        ennemy_created.val = false;
    }

    for (mut transform, mut enemy) in query.iter_mut() {
        if let Some(player) = list_player.list.get(&enemy.id) {
            enemy.position = player.position.clone();
            transform.translation = Vec3::new(
                enemy.position.x,
                enemy.position.y - 0.2,
                enemy.position.z
            );
            transform.rotate_local_y(-player.vision.0 * 0.002);
        }
    }
}