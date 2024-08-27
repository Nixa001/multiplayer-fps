use renet::transport::{ ServerAuthentication, ServerConfig, NetcodeServerTransport };
use renet::{ ConnectionConfig, DefaultChannel, RenetServer, ServerEvent };
use store::GameState;
use std::net::{ SocketAddr, UdpSocket };
use std::time::SystemTime;
use std::thread::*;
use store::{ PROTOCOL_ID, GAME_FPS, * };
use bincode::*;
use server::*;
use local_ip_address::local_ip;

fn main() {
    let ip_address = match local_ip() {
        Ok(ip) => ip.to_string(), // Convertit l'adresse IP en chaîne de caractères
        Err(e) => {
            eprintln!("❌ Error while retrieving local Ip address: {}", e);
            return;
        }
    };

    let port = 8080;
    let ip_with_port = format!("{}:{}", ip_address, port);
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
    let mut game_state = GameState::default();
    let lvl = get_level();
    game_state.set_lvl(lvl);

    println!("🕹 maze server listening on {} 📡", server_addr);

    loop {
        // Receive new messages and update clients at desired fps
        server.update(GAME_FPS);
        transport.update(GAME_FPS, &mut server).expect("error while transporting from server");

        while let Some(event) = server.get_event() {
            match event {
                ServerEvent::ClientConnected { client_id } => {
                    // * ------ connection logic
                    let player_id = game_state.generate_id();
                    let spawn_coord = game_state.random_spawn();
                    let event = GameEvent::PlayerJoined {
                        player_id,
                        name: "player".to_string(),
                        position: spawn_coord.clone(),
                        client_id: client_id.raw(),
                    };
                    println!("🟢 [{}] joined the server.", player_id);
                    server.broadcast_message_except(
                        client_id,
                        DefaultChannel::ReliableOrdered,
                        serialize(&event).expect("error while serializing event")
                    );

                    let id_event = GameEvent::Spawn {
                        player_id,
                        position: spawn_coord.clone(),
                        lvl: game_state.lvl,
                    };

                    server.send_message(
                        client_id,
                        DefaultChannel::ReliableOrdered,
                        serialize(&id_event).expect("error while sending id to client")
                    );

                    game_state.consume(&event, client_id.raw());

                    if game_state.players.len() == PLAYER_LIMIT {
                        let event = GameEvent::BeginGame;
                        game_state.consume(&event, client_id.raw());
                        server.broadcast_message(0, serialize(&event).unwrap());
                        println!("🟩 The game has begun");
                    }
                    break;
                }

                ServerEvent::ClientDisconnected { client_id, reason } => {
                    // * -------- disconnection logic
                    let player_id = game_state.get_player_id(client_id.raw());
                    // First consume a disconnect event
                    let event = GameEvent::PlayerDisconnected { player_id };
                    game_state.consume(&event, client_id.raw());
                    server.broadcast_message(0, serialize(&event).unwrap());
                    println!("🔻 Player [{}] disconnected due to \"{}\"", player_id, reason);

                    if game_state.players.len() == 1 && game_state.stage == Stage::InGame {
                        let event = GameEvent::EndGame;
                        game_state.consume(&event, client_id.raw());
                        server.broadcast_message(0, serialize(&event).unwrap());
                        println!("🟥 Game has ended");
                    }
                }
            }
        }

        // ! Receive GameEvents from clients. Broadcast valid events.
        for client_id in server.clients_id().into_iter() {
            while
                let Some(message) = server.receive_message(
                    client_id,
                    DefaultChannel::ReliableOrdered
                )
            {
                if let Ok(event) = deserialize::<GameEvent>(&message) {
                    if game_state.validate(&event, client_id.raw()) {
                        let broad_event = game_state.consume(&event, client_id.raw());
                        println!("[EVENT]: Client {} sent:\n\t{:#?}", client_id, broad_event);
                        match broad_event {
                            GameEvent::PlayerMove { player_id: _, at: _ } => {
                                server.broadcast_message_except(
                                    client_id,
                                    DefaultChannel::ReliableOrdered,
                                    serialize(&broad_event).unwrap()
                                );
                            }
                            _ => {
                                server.broadcast_message(
                                    DefaultChannel::ReliableOrdered,
                                    serialize(&broad_event).unwrap()
                                );
                            }
                        }

                        // ^Determine if a player has won the game at each request
                        if let Some(winner) = game_state.determine_winner() {
                            let event = GameEvent::EndGame;
                            game_state.stage = Stage::Ended;
                            server.broadcast_message(
                                DefaultChannel::ReliableOrdered,
                                serialize(&event).unwrap()
                            );
                            println!("[INFO]: player with id [{}] won !", winner);
                        }
                    } else {
                        eprintln!("❌ Player {} sent invalid event:\n\t{:#?}", client_id, event);
                    }
                }
            }
        }
        transport.send_packets(&mut server);
        sleep(GAME_FPS);
    }
}
