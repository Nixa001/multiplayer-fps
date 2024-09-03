use bevy::asset::{ Assets };
use bevy::log::{ error, info, warn };
use bevy::math::Vec3;
use bevy::pbr::StandardMaterial;
use bevy::prelude::{ Commands, Mesh, Query, ResMut, Resource, Transform, With };
use bevy_renet::renet::transport::ClientAuthentication;
use bevy_renet::renet::transport::NetcodeClientTransport;
use bevy_renet::renet::{ ConnectionConfig, DefaultChannel, RenetClient };
use bincode::{ deserialize, serialize };
use std::collections::HashMap;
use std::{
    io::{ self, Write },
    net::{ SocketAddr, UdpSocket },
    process::*,
    thread::sleep,
    time::SystemTime,
};
use store::{ GameEvent, Players, GAME_FPS, NBR_OF_LIVES, PROTOCOL_ID };
mod enemys;
mod games;
mod player;
mod player_2d;
mod playing_field;
use crate::player::player::Player;

#[derive(Default, Resource, Debug)]
pub struct ListPlayer {
    pub list: HashMap<u8, Players>,
}

#[derive(Debug, Default, Resource)]
pub struct PositionInitial {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[derive(Debug, Default, Resource)]
pub struct Counter {
    pub val: i32,
}
#[derive(Debug, Default, Resource)]
pub struct LifeCounter {
    pub val: u8,
}

impl LifeCounter {
    pub fn new() -> Self {
        Self { val: NBR_OF_LIVES }
    }

    pub fn reduce(&mut self) {
        self.val -= 1;
    }
}
#[derive(Debug, Resource)]
pub struct EnnemyCreated {
    pub val: bool,
}

#[derive(Debug, Default, Resource)]
pub struct GameTimer {
    pub sec: i32,
}
#[derive(Debug, Resource)]
pub struct GameState {
    pub is_waiting: bool,
    pub has_started: bool,
    pub has_ended: bool,
}
impl GameState {
    pub fn new() -> Self {
        Self {
            is_waiting: true,
            has_ended: false,
            has_started: false,
        }
    }
    pub fn start_game(&mut self) {
        self.is_waiting = false;
        self.has_started = true;
        self.has_ended = false;
    }

    pub fn end_game(&mut self) {
        self.is_waiting = false;
        self.has_started = false;
        self.has_ended = true;
    }
}
#[derive(Resource)]
pub struct PlayerSpawnInfo {
    pub player_id: Option<u8>,
    pub position: Option<Vec3>,
}

pub fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn setup_networking(
    server_addr: &SocketAddr,
    username: &str
) -> (RenetClient, NetcodeClientTransport) {
    let client = RenetClient::new(ConnectionConfig::default());
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let client_id = current_time.as_millis() as u64;

    let mut user_data = [0u8; 256];
    let username_len = username.len() as u64;
    user_data[0..8].copy_from_slice(&username_len.to_le_bytes());
    user_data[8..8 + username.len()].copy_from_slice(username.as_bytes());

    let authentication = ClientAuthentication::Unsecure {
        server_addr: *server_addr,
        client_id,
        user_data: Some(user_data),
        protocol_id: PROTOCOL_ID,
    };

    let socket = UdpSocket::bind("0.0.0.0:5000").unwrap_or_else(|_| {
        error!(
            "❌ Address already in use! Only one client can run on the same machine used as server"
        );
        exit(1);
    });

    let transport = NetcodeClientTransport::new(current_time, authentication, socket).expect(
        "Failed to create transport"
    );

    (client, transport)
}

pub fn handle_connection(
    mut client: ResMut<RenetClient>,
    mut lives: ResMut<LifeCounter>,
    mut transport: ResMut<NetcodeClientTransport>,
    _player_query: Query<&mut Transform, With<Player>>,
    spawn_info: ResMut<PlayerSpawnInfo>,
    commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut location: ResMut<PositionInitial>,
    mut liste_player: ResMut<ListPlayer>,
    mut game_state: ResMut<GameState>,
    mut game_timer: ResMut<GameTimer>
) {
    client.update(GAME_FPS);
    if transport.update(GAME_FPS, &mut client).is_err() {
        warn!("Server is unavailable");
        client.disconnect_due_to_transport();
        exit(1);
    }

    if client.is_connected() {
        handle_server_messages(
            &mut client,
            &mut lives,
            commands,
            &mut meshes,
            &mut materials,
            spawn_info,
            &mut location,
            &mut liste_player,
            &mut game_state,
            &mut game_timer
        );
    }

    transport.send_packets(&mut client).expect("Error while sending packets to server");
    // sleep(GAME_FPS);
}

pub fn handle_server_messages(
    client: &mut ResMut<RenetClient>,
    lives: &mut ResMut<LifeCounter>,
    mut commands: Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    mut spawn_info: ResMut<PlayerSpawnInfo>,
    location: &mut ResMut<PositionInitial>,
    liste_player: &mut ResMut<ListPlayer>,
    game_state: &mut ResMut<GameState>,
    game_timer: &mut ResMut<GameTimer>
) {
    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        if let Ok(event) = deserialize::<GameEvent>(&message) {
            match event {
                GameEvent::Spawn { player_id, position, lvl } => {
                    info!(
                        "i am player [{}] located at \"{}°- {}°- {}°\" on level: {}",
                        player_id,
                        position.x,
                        position.y,
                        position.z,
                        lvl
                    );

                    // Mettre à jour la position du joueur
                    // if let Ok(mut transform) = player_query.get_single_mut() {
                    //     transform.translation = Vec3::new(position.x, position.y, position.z);
                    // }
                    location.x = position.x;
                    location.y = position.y;
                    location.z = position.z;
                    // Stocker les informations de spawn
                    spawn_info.player_id = Some(player_id);
                    spawn_info.position = Some(Vec3::new(position.x, position.y, position.z));

                    playing_field::playing_field::create_maze(
                        &mut commands,
                        meshes,
                        materials,
                        format!("Map{}", lvl).as_str()
                    );
                }
                GameEvent::PlayerJoined { player_id, name: _, position: _, .. } => {
                    // ! implement logic here
                    info!("[{}] joined the war ", player_id);
                }

                GameEvent::PlayerMove { player_list, .. } => {
                    //println!("****************FROM SERVER => {:#?}***************", player_list);
                    liste_player.list = player_list;
                }
                GameEvent::Timer { duration } => {
                    game_timer.sec = duration as i32;
                }

                GameEvent::BeginGame { player_list } => {
                    game_state.start_game();
                    info!("Game has begun with warriors => {:#?}", player_list);
                }

                GameEvent::AccessForbidden => {
                    info!("❌ Oops ! ongoing game...");
                    exit(1);
                }

                GameEvent::EndGame => {
                    info!("🥉 i am the winner");
                    println!("💣💣💣💣💣💣💣💣💣💣💣💣💣💣💣💣💣💣💣💣💣💣");
                    println!("💣                                                  💣");
                    println!("💣          👑 YOU WON ! THE WARRIOR  👑           💣");
                    println!("💣                                                  💣");
                    println!("💣💣💣💣💣💣💣💣💣💣💣💣💣💣💣💣💣💣💣💣💣💣");
                }
                GameEvent::Impact { id } => {
                    lives.reduce();
                    if lives.val == 0 {
                        let death_event = GameEvent::Death { player_id: id };

                        client.send_message(
                            DefaultChannel::ReliableOrdered,
                            serialize(&death_event).unwrap()
                        );
                        println!("❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌");
                        println!("❌                                                  ❌");
                        println!("❌           😔 GAME OVER TRY AGAIN WARRIOR 😔     ❌");
                        println!("❌                                                  ❌");
                        println!("❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌❌");
                        client.disconnect();
                        game_state.end_game();
                    }
                }

                GameEvent::Death { player_id } => {
                    info!("🔻 [{}] has died", player_id);
                    // liste_player.list.remove(&player_id);
                }

                // ! do the same for other events
                _ => {
                    println!("received event from server => {:?}", event);
                }
            }
            // Handle server events here
        }
        // info!("Move detected = > {:#?}", liste_player.list);
    }
}
