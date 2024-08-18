use renet::transport::{ ServerAuthentication, ServerConfig, NetcodeServerTransport };
use renet::{ ConnectionConfig, RenetServer, ServerEvent };
use store::GameState;
use std::net::{ SocketAddr, UdpSocket };
use std::time::{ Duration, SystemTime };
use std::thread::*;
use store::*;
use bincode::*;
use server::*;
use local_ip_address::local_ip;

pub const PROTOCOL_ID: u64 = 1582;

fn main() {
    env_logger::init();

    let ip_adress = match local_ip() {
        Ok(ip) => ip.to_string(), // Convertit l'adresse IP en chaîne de caractères
        Err(e) => {
            eprintln!("Erreur lors de la récupération de l'adresse IP : {}", e);
            return;
        }
    };

    let port = 8080;
    let ip_with_port = format!("{}:{}", ip_adress, port);
    let server_addr: SocketAddr = ip_with_port.parse().unwrap();
    let socket: UdpSocket = UdpSocket::bind(server_addr).unwrap();

    let mut server = RenetServer::new(ConnectionConfig::default());
    let server_config = ServerConfig {
        current_time: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap(),
        max_clients: PLAYER_LIMIT,
        protocol_id: PROTOCOL_ID,
        public_addresses: vec![server_addr],
        authentication: ServerAuthentication::Unsecure,
    };
    let mut transport = NetcodeServerTransport::new(server_config, socket).unwrap();

    println!("🕹 maze server listening on {}", server_addr);

    //let mut last_updated = Instant::now();
    let mut game_state = GameState::default();

    loop {
        // Update server time
        let delta_time = Duration::from_millis(16);
        // Receive new messages and update clients
        server.update(delta_time);
        transport.update(delta_time, &mut server).expect("error while transporting from server");

        while let Some(event) = server.get_event() {
            match event {
                ServerEvent::ClientConnected { client_id } => {
                    // Tell the recently joined player about the other player
                    for (player_id, _player) in game_state.players.iter() {
                        let event = GameEvent::PlayerJoined {
                            player_id: *player_id,
                            name: "player".to_string(),
                        };
                        println!("innit");
                        server.send_message(client_id, 0, serialize(&event).unwrap());
                    }

                    //let name = name_from_user_data(&user_data);
                    // Add the new player to the game
                    let event = GameEvent::PlayerJoined {
                        player_id: client_id.raw() as u8,
                        name: "player".to_string(),
                    };
                    game_state.consume(&event);

                    // Tell all players that a new player has joined
                    server.broadcast_message(0, serialize(&event).unwrap());

                    println!("[{}] joined the server.", client_id.raw());
                    if game_state.players.len() == PLAYER_LIMIT {
                        let event = GameEvent::BeginGame;
                        game_state.consume(&event);
                        server.broadcast_message(0, bincode::serialize(&event).unwrap());
                        println!("The game gas begun");
                    }
                    break;
                }

                ServerEvent::ClientDisconnected { client_id, reason } => {
                    // First consume a disconnect event
                    let event = GameEvent::PlayerDisconnected { player_id: client_id.raw() as u8 };
                    println!("bef => {:#?}", game_state.players);
                    game_state.consume(&event);
                    server.broadcast_message(0, bincode::serialize(&event).unwrap());
                    println!("Client [{}] disconnected due to \"{}\"", client_id.raw(), reason);

                    if game_state.players.len() == 1 {
                        let event = GameEvent::EndGame;
                        game_state.consume(&event);
                        server.broadcast_message(0, bincode::serialize(&event).unwrap());
                        println!("Game has ended");
                    }
                }
            }
        }
        // Receive GameEvents from clients. Broadcast valid events.
        for client_id in server.clients_id().into_iter() {
            while let Some(message) = server.receive_message(client_id, 0) {
                if let Ok(event) = deserialize::<GameEvent>(&message) {
                    if game_state.validate(&event) {
                        game_state.consume(&event);
                        println!("Player {} sent:\n\t{:#?}", client_id, event);
                        server.broadcast_message(0, serialize(&event).unwrap());

                        // Determine if a player has won the game
                        if let Some(winner) = game_state.determine_winner() {
                            let event = GameEvent::EndGame;
                            server.broadcast_message(0, bincode::serialize(&event).unwrap());
                            println!("player with id [{}] won !", winner);
                        }
                    } else {
                        println!("Player {} sent invalid event:\n\t{:#?}", client_id, event);
                    }
                }
            }
        }

        // server.send_packets().unwrap();
        transport.send_packets(&mut server);
        sleep(delta_time);
    }
}
