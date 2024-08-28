use bevy::asset::{ AssetServer, Assets };
use bevy::log::{ error, info, warn };
use bevy::prelude::{ Commands, Mesh, Query, Res, ResMut, Resource, Transform, With };
use bevy_renet::renet::transport::ClientAuthentication;
use bevy_renet::renet::transport::NetcodeClientTransport;
use bevy_renet::renet::{ ConnectionConfig, DefaultChannel, RenetClient };
use bincode::deserialize;
use std::{
    io::{ self, Write },
    net::{ SocketAddr, UdpSocket },
    thread::sleep,
    time::SystemTime,
    process::*,
};
use bevy::math::Vec3;
use bevy::pbr::StandardMaterial;
use store::{ GameEvent, GAME_FPS, PROTOCOL_ID };
mod player;
mod player_2d;
mod playing_field;
use crate::player::player::Player;

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
        std::process::exit(1);
    });

    let transport = NetcodeClientTransport::new(current_time, authentication, socket).expect(
        "Failed to create transport"
    );

    (client, transport)
}

pub fn handle_connection(
    mut client: ResMut<RenetClient>,
    mut transport: ResMut<NetcodeClientTransport>,
    mut player_query: Query<&mut Transform, With<Player>>,
    mut spawn_info: ResMut<PlayerSpawnInfo>,
    commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>
) {
    client.update(GAME_FPS);
    if transport.update(GAME_FPS, &mut client).is_err() {
        warn!("Server is unavailable");
        client.disconnect_due_to_transport();
        std::process::exit(1);
    }

    if client.is_connected() {
        handle_server_messages(
            &mut client,
            commands,
            &asset_server,
            &mut meshes,
            &mut materials,
            player_query,
            spawn_info
        );

        // while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        //     if let Ok(event) = deserialize::<GameEvent>(&message) {
        //         match event {
        //             GameEvent::Spawn {
        //                 player_id,
        //                 position,
        //                 lvl,
        //             } => {
        //                 info!(
        //                     "i am player [{}] located at \"{}°- {}°- {}°\" on level: {}",
        //                     player_id, position.x, position.y, position.z, lvl
        //                 );
        //
        //
        //                 // Créer le labyrinthe
        //
        //                 // playing_field::playing_field::create_maze(&mut commands, meshes, materials, format!("Map{}", lvl).as_str());
        //             },
        //             // ... autres cas
        //             _ => panic!(),
        //         }
        //     }
        // }
        // Example of sending a message to the server:
        // client.send_message(DefaultChannel::ReliableOrdered, serialize(&event).unwrap());
    }

    transport.send_packets(&mut client).expect("Error while sending packets to server");
    // sleep(GAME_FPS);
}

pub fn handle_server_messages(
    client: &mut ResMut<RenetClient>,
    mut commands: Commands,
    asset_server: &Res<AssetServer>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    mut spawn_info: ResMut<PlayerSpawnInfo>
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
                    // setup_player_and_camera(
                    //     &mut commands,
                    //     asset_server,
                    //     player_id,
                    //     position.x,
                    //     position.y,
                    //     position.z,
                    // );
                    // Mettre à jour la position du joueur
                    if let Ok(mut transform) = player_query.get_single_mut() {
                        eprintln!("ddfdf");
                        transform.translation = Vec3::new(position.x, position.y, position.z);
                    }

                    // Stocker les informations de spawn
                    spawn_info.player_id = Some(player_id);
                    spawn_info.position = Some(Vec3::new(position.x, position.y, position.z));
                    println!("received 💢 {:?}", position);

                    playing_field::playing_field::create_maze(
                        &mut commands,
                        meshes,
                        materials,
                        format!("Map{}", lvl).as_str()
                    );
                }
                GameEvent::PlayerJoined { player_id, name, position, .. } => {
                    // ! implement logic here
                    info!(
                        "{} [{}] joined the party and is located at \"{}°- {}°- {}°\" ",
                        name,
                        player_id,
                        position.x,
                        position.y,
                        position.z
                    );
                }

                GameEvent::PlayerMove { player_list, .. } => {
                    info!("Move detected = > {:#?}", player_list);
                }
                GameEvent::Timer { duration } => {
                    info!("🕗 timer tickling => {}", duration);
                }

                GameEvent::BeginGame { player_list } => {
                    info!("Game has begun with warriors => {:#?}", player_list);
                }

                GameEvent::AccessForbidden => {
                    info!("❌ Oops ! ongoing game...");
                    exit(1);
                }

                // ! do the same for other events
                _ => {
                    println!("received event from server => {:?}", event);
                }
            }
            // Handle server events here
        }
    }
}
