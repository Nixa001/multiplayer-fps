use bevy::log::{ error, info, warn };
use bevy::prelude::ResMut;
use bevy_renet::renet::transport::ClientAuthentication;
use bevy_renet::renet::{ ConnectionConfig, DefaultChannel, RenetClient };
use bevy_renet::renet::transport::NetcodeClientTransport;
use bincode::deserialize;
use std::process::exit;
use std::{ io::{ self, Write }, net::{ SocketAddr, UdpSocket }, thread::sleep, time::SystemTime };
use store::{ GameEvent, GAME_FPS, PROTOCOL_ID };

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
    mut transport: ResMut<NetcodeClientTransport>
) {
    client.update(GAME_FPS);
    if transport.update(GAME_FPS, &mut client).is_err() {
        warn!("Server is unavailable");
        client.disconnect_due_to_transport();
        std::process::exit(1);
    }

    if client.is_connected() {
        handle_server_messages(&mut client);
        // Example of sending a message to the server:
        // client.send_message(DefaultChannel::ReliableOrdered, serialize(&event).unwrap());
    }

    transport.send_packets(&mut client).expect("Error while sending packets to server");
    sleep(GAME_FPS);
}

pub fn handle_server_messages(client: &mut ResMut<RenetClient>) {
    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        if let Ok(event) = deserialize::<GameEvent>(&message) {
            match event {
                GameEvent::Spawn { player_id, position, lvl } => {
                    info!(
                        "i am player [{}] located at \"{}°-{}°-{}°\" on level: {}",
                        player_id,
                        position.x,
                        position.y,
                        position.z,
                        lvl
                    );
                }
                GameEvent::PlayerJoined { player_id, name, position, .. } => {
                    // ! implement logic here
                    info!(
                        "{} [{}] joined the party and is located at \"{}°-{}°-{}°\" ",
                        name,
                        player_id,
                        position.x,
                        position.y,
                        position.z
                    );
                }

                GameEvent::Timer { duration } => {
                    info!("🕗 timer tickling => {}", duration);
                }

                GameEvent::BeginGame => {
                    info!("Game has begun");
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
