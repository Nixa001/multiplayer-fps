use bevy::prelude::*;
use store::Position;
use multiplayer_fps::Counter;
use crate::ListPlayer;
use crate::player::player::Player;

#[derive(Component)]
pub struct Enemy {
    pub id: u8,
    pub name: String,
    pub position: Position,
}

impl Enemy {
    pub fn new(id: u8, name: String, position: Position) -> Self {
        Enemy {
            id,
            name,
            position,
        }
    }
}

pub fn create_enemys(
    commands: &mut Commands,
    list_player: &ListPlayer,
    asset_server: &AssetServer,
) {
    for (&id, player) in list_player.list.iter() {
        let player_handle: Handle<Scene> = asset_server.load("soldier/Soldier.glb#Scene0");
        // let player_handle:Handle<Scene> = asset_server.load("armes/Soldier.glb#Scene0");
        let player_entity = commands
            .spawn((
                Enemy::new(id, format!("Enemy_{}", id), player.position.clone()),
                SceneBundle {
                    scene: player_handle,
                    transform: Transform::from_xyz(player.position.x, player.position.y, player.position.z).with_scale(Vec3::splat(0.4)),
                    ..default()
                },
        ));
    }
}

pub fn update_enemys_position(
    mut query: Query<(&mut Transform, &mut Enemy)>,
    mut commands: Commands,
    list_player: Res<ListPlayer>,
    asset_server: Res<AssetServer>,
    mut counter: ResMut<Counter>,
) {
    println!("------------Enemys----------- {:?}", list_player.list);
    let count = counter.val;
    // if count < 1 {
        
        create_enemys(&mut commands, &list_player, &asset_server);
    // }
    counter.val += 1;
    for (mut transform, mut enemy) in query.iter_mut() {
        if let Some(player) = list_player.list.get(&enemy.id) {
            enemy.position = player.position.clone();
            transform.translation = Vec3::new(
                enemy.position.x,
                enemy.position.y,
                enemy.position.z
            );
        }
    }
}