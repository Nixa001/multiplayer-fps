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


use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn create_enemys(
    commands: &mut Commands,
    list_player: &ListPlayer,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>
) {
    println!("------------Enemys-------{:?}", list_player.list);
    
    for (&id, player) in list_player.list.iter() {
        let soldier_entity = commands.spawn(SpatialBundle::default())
            .insert(Enemy::new(id, format!("Enemy_{}", id), player.position.clone()))
            .insert(RigidBody::KinematicPositionBased)
            .insert(Collider::capsule(0.5, 0.2))
            .insert(Velocity::default())
            .insert(Name::new(format!("Enemy_{}", id)))
            .id();

        // Corps (torse)
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule { 
                radius: 0.2, 
                rings: 3, 
                depth: 0.5, 
                latitudes: 8, 
                longitudes: 16, 
                uv_profile: shape::CapsuleUvProfile::Aspect 
            })),
            material: materials.add(Color::DARK_GRAY.into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        }).set_parent(soldier_entity);

        // Tête
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere { 
                radius: 0.15, 
                sectors: 16, 
                stacks: 16 
            })),
            material: materials.add(Color::BEIGE.into()),
            transform: Transform::from_xyz(0.0, 0.9, 0.0),
            ..default()
        }).set_parent(soldier_entity);

        // Bras gauche
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule { 
                radius: 0.05, 
                rings: 3, 
                depth: 0.4, 
                latitudes: 8, 
                longitudes: 16, 
                uv_profile: shape::CapsuleUvProfile::Aspect 
            })),
            material: materials.add(Color::DARK_GRAY.into()),
            transform: Transform::from_xyz(-0.25, 0.5, 0.0)
                .with_rotation(Quat::from_rotation_x(0.5)),
            ..default()
        }).set_parent(soldier_entity);

        // Bras droit (tenant l'arme)
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule { 
                radius: 0.05, 
                rings: 3, 
                depth: 0.4, 
                latitudes: 8, 
                longitudes: 16, 
                uv_profile: shape::CapsuleUvProfile::Aspect 
            })),
            material: materials.add(Color::DARK_GRAY.into()),
            transform: Transform::from_xyz(0.25, 0.5, 0.0)
                .with_rotation(Quat::from_rotation_z(-0.5)),
            ..default()
        }).set_parent(soldier_entity);

        // AK-47 (représentation simplifiée)
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(0.5, 0.1, 0.05))),
            material: materials.add(Color::BLACK.into()),
            transform: Transform::from_xyz(0.4, 0.4, 0.2)
                .with_rotation(Quat::from_rotation_y(0.2)),
            ..default()
        }).set_parent(soldier_entity);

        // Positionnement global du soldat
        commands.entity(soldier_entity).insert(Transform::from_xyz(
            player.position.x,
            player.position.y,
            player.position.z
        ));

        println!("Spawned enemy with ID: {:?}", soldier_entity);
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
        create_enemys(&mut commands, &list_player, &asset_server);
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