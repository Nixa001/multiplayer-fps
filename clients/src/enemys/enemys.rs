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
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    println!("------------Enemys-------{:?}", list_player.list);
    
    for (&id, player) in list_player.list.iter() {
        let enemy = Enemy::new(id, format!("Enemy_{}", id), player.position.clone());
        let transform = Transform::from_xyz(
            player.position.x,
            player.position.y,
            player.position.z
        );

        // Corps
        let body = commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule {
                radius: 0.25,
                rings: 0,
                depth: 1.0,
                latitudes: 16,
                longitudes: 32,
                uv_profile: shape::CapsuleUvProfile::Uniform
            })),
            material: materials.add(Color::GRAY.into()),
            transform: transform.with_scale(Vec3::new(0.5, 0.5, 0.5)),
            ..default()
        }).id();

        // Tête
        let head = commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 0.3,
                sectors: 32,
                stacks: 16,
            })),
            material: materials.add(Color::ANTIQUE_WHITE.into()),
            transform: Transform::from_xyz(0.0, 0.8, 0.0),
            ..default()
        }).id();

        // Arme (AK-47 simplifié)
        let weapon = commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(0.1, 0.3, 1.0))),
            material: materials.add(Color::BLACK.into()),
            transform: Transform::from_xyz(0.3, 0.0, 0.5).with_rotation(Quat::from_rotation_x(-0.2)),
            ..default()
        }).id();

        // Assembler le soldat
        let soldier = commands.spawn((
            enemy,
            SpatialBundle::from_transform(transform),
            RigidBody::KinematicPositionBased,
            Collider::capsule(Vec3::new(0.0, -0.5, 0.0), Vec3::new(0.0, 0.5, 0.0), 0.25),
            Velocity::default(),
        ))
        .add_child(body)
        .add_child(head)
        .add_child(weapon)
        .insert(Name::new(format!("Enemy_{}", id)))
        .id();

        println!("Spawned enemy with ID: {:?}", soldier);
    }
}

pub fn update_enemys_position(
    mut query: Query<(&mut Transform, &mut Enemy)>,
    mut commands: Commands,
    list_player: Res<ListPlayer>,
    game_state: Res<GameState>,
    mut ennemy_created: ResMut<EnnemyCreated>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    if game_state.has_started && ennemy_created.val {
        println!("❌❌❌❌");
        create_enemys(&mut commands, &list_player, &mut meshes, &mut materials);
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