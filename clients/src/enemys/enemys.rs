use bevy::prelude::*;
use store::Position;
use crate::{ListPlayer};

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
    mut commands: Commands,
    list_player: Res<ListPlayer>,
    asset_server: Res<AssetServer>,
) {
    for (&id, player) in list_player.list.iter() {
        let enemy_model: Handle<Scene> = asset_server.load("soldier/Soldier.glb#Scene0");

        commands.spawn((
            Enemy::new(id, format!("Enemy_{}", id), player.position.clone()),
            SceneBundle {
                scene: enemy_model,
                transform: Transform::from_translation(Vec3::new(
                    player.position.x,
                    player.position.y,
                    player.position.z
                )),
                ..default()
            },
        ));
    }
}

pub fn update_enemys_position(
    mut query: Query<(&mut Transform, &mut Enemy)>,
    list_player: Res<ListPlayer>,
) {
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