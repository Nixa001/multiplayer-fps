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
    pub lives:u8,
}

impl Enemy {
    pub fn new(id: u8, name: String, position: Position) -> Self {
        Enemy {
            id,
            name,
            position,
            lives:3
        }
    }
}

pub fn create_enemys(
    commands: &mut Commands,
    list_player: &ListPlayer,
    asset_server: &AssetServer
) {
    println!("------------Enemys-------{:?}", list_player.list);
    for (&id, player) in list_player.list.iter() {
        let player_handle: Handle<Scene> = asset_server.load("soldier/soldier2.glb#Scene0");
        // let player_handle:Handle<Scene> = asset_server.load("armes/Soldier.glb#Scene0");
        let player_entity = commands.spawn((
            Enemy::new(id, format!("Enemy_{}", id), player.position.clone()),
            SceneBundle {
                scene: player_handle,
                transform: Transform::from_xyz(
                    player.position.x,
                    player.position.y,
                    player.position.z
                ).with_scale(Vec3::splat(0.02)),
                ..default()
            },

            RigidBody::KinematicPositionBased,
            Collider::ball(10.5),
            Velocity::default(), // Assurez-vous que cette ligne est présente
            LockedAxes::ROTATION_LOCKED,
            GravityScale(0.0),


            // RigidBody::KinematicPositionBased, // ou Kinematic selon le comportement souhaité
            // Collider::cuboid(2.5, 2.5, 2.5),
        ));
    }
}

pub fn update_enemys_position(
    mut query: Query<(&mut Transform, &mut Enemy)>,
    mut commands: Commands,
    list_player: Res<ListPlayer>,
    asset_server: Res<AssetServer>,
    game_state: Res<GameState>,
    mut ennemy_created: ResMut<EnnemyCreated>
    //mut counter: ResMut<Counter>
) {
    //let count = counter.val;
    // if !list_player.list.is_empty() {
    if game_state.has_started && ennemy_created.val {
        println!("❌❌❌❌");
        create_enemys(&mut commands, &list_player, &asset_server);
        ennemy_created.val = false;
    }
    // }
    //println!("---- Counte ------  = {}", count);
    //counter.val += 1;
    for (mut transform, mut enemy) in query.iter_mut() {
        if let Some(player) = list_player.list.get(&enemy.id) {
            enemy.position = player.position.clone();
            transform.translation = Vec3::new(
                enemy.position.x,
                enemy.position.y - 0.2,
                enemy.position.z
            );
            transform.rotate_local_y(-player.vision.0* 0.002);
            //transform.rotate_x(player.vision.0 * 0.002);
        }
    }
}
