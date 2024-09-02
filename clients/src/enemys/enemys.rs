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
) {
    println!("------------Enemys-------{:?}", list_player.list);
    let player_handle: Handle<Scene> = asset_server.load("soldier/guy.glb#Scene0");

    for (&id, player) in list_player.list.iter() {
        let enemy = Enemy::new(id, format!("Enemy_{}", id), player.position.clone());
        let transform = Transform::from_xyz(
            player.position.x,
            player.position.y,
            player.position.z
        ).with_scale(Vec3::splat(0.02));

        // Ajuster la taille du collider en fonction de l'échelle du modèle
        let collider_height = 1.3 * 0.02; // Hauteur originale * échelle
        let collider_radius = 0.15 * 0.02; // Rayon original * échelle

        let player_entity = commands.spawn((
            enemy,
            SceneBundle {
                scene: player_handle.clone(),
                transform,
                ..default()
            },
            RigidBody::KinematicPositionBased,
            Collider::capsule(Vec3::new(0.0, -collider_height/2.0, 0.0), Vec3::new(0.0, collider_height/2.0, 0.0), collider_radius),
            Velocity::default(),
            // DebugCollision::default(), // Ajouter ceci pour voir le collider
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