use renet::{ RenetConnectionConfig, RenetServer, ServerAuthentication, ServerConfig, ServerEvent };
use store::GameState;
use std::fmt::format;
use std::net::{ SocketAddr, UdpSocket };
use std::time::{ Duration, Instant, SystemTime };
use std::thread::*;
use store::*;
use bincode::*;
use server::*;
use local_ip_address::local_ip;

pub const PROTOCOL_ID: u64 = 1582;

fn main() {
    env_logger::init();

    let ip_adress = match local_ip() {
        Ok(ip) => ip.to_string(),  // Convertit l'adresse IP en chaîne de caractères
        Err(e) => {
            eprintln!("Erreur lors de la récupération de l'adresse IP : {}", e);
            return;
        }
    };


    let port = 8080;
    let ip_with_port = format!("{}:{}", ip_adress, port);
    
    let server_addr: SocketAddr = ip_with_port.parse().unwrap();
    let mut server: RenetServer = RenetServer::new(
        SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap(),
        ServerConfig::new(PLAYER_LIMIT, PROTOCOL_ID, server_addr, ServerAuthentication::Unsecure),
        RenetConnectionConfig::default(),
        UdpSocket::bind(server_addr).unwrap()
    ).unwrap();

    println!("🕹 maze server listening on {}", server_addr);

    let mut last_updated = Instant::now();
    let mut game_state = GameState::default();

    loop {
        // Update server time
        let now = Instant::now();
        server.update(now - last_updated).unwrap();
        last_updated = now;

        while let Some(event) = server.get_event() {
            match event {
                ServerEvent::ClientConnected(id, user_data) => {
                    // Tell the recently joined player about the other player
                    for (player_id, player) in game_state.players.iter() {
                        let event = GameEvent::PlayerJoined {
                            player_id: *player_id,
                            name: player.name.clone(),
                        };
                        println!("innit");
                        server.send_message(id, 0, serialize(&event).unwrap());
                    }
                    let name = name_from_user_data(&user_data);
                    // Add the new player to the game
                    let event = GameEvent::PlayerJoined {
                        player_id: id as u8,
                        name: name.clone(),
                    };
                    game_state.consume(&event);

                    // Tell all players that a new player has joined
                    server.broadcast_message(0, serialize(&event).unwrap());

                    println!("{}-[{}] joined the server.", name, id);
                    if game_state.players.len() == PLAYER_LIMIT {
                        let event = GameEvent::BeginGame;
                        game_state.consume(&event);
                        server.broadcast_message(0, bincode::serialize(&event).unwrap());
                        println!("The game gas begun");
                    }
                }

                ServerEvent::ClientDisconnected(id) => {
                    // First consume a disconnect event
                    let event = GameEvent::PlayerDisconnected { player_id: id as u8 };
                    game_state.consume(&event);
                    server.broadcast_message(0, bincode::serialize(&event).unwrap());
                    println!("Client {} disconnected", id);

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

        server.send_packets().unwrap();
        sleep(Duration::from_millis(50));
    }
}
