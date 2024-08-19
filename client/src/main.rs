use bevy::prelude::*;
use bevy_renet::{ renet::*, transport::NetcodeClientPlugin, RenetClientPlugin };
use store::{ PROTOCOL_ID, GAME_FPS, * };
use bincode::*;
use transport::{ ClientAuthentication, NetcodeClientTransport };
use std::{ net::{ SocketAddr, UdpSocket }, time::SystemTime, thread::* };
use std::io::{ self, Write, * };

fn main() {
    let mut input = String::new();

    print!("Enter server IP address: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut input).unwrap();
    let server_ip = input.trim().to_string();
    if server_ip.is_empty() {
        eprintln!("❌ Please provide an IP address");
        return;
    }
    if server_ip.parse::<SocketAddr>().is_err() {
        eprintln!("❌ invalid Ip address!");
        return;
    }
    input.clear();

    print!("Enter your username: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let username = input.trim().to_string();

    if username.is_empty() {
        eprintln!("❌ Please provide a username");
        return;
    }
    if username.len() > 256 - 8 {
        eprintln!("Username is too big");
        return;
    }

    //--------- app
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "test".into(),
                resolution: (100.0, 100.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        })
    );
    app.add_plugins(RenetClientPlugin);

    let client = RenetClient::new(ConnectionConfig::default());
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let client_id = current_time.as_millis() as u64;
    app.insert_resource(client);
    // Setup the transport layer

    app.add_plugins(NetcodeClientPlugin);
    let mut user_data = [0u8; 256];
    user_data[0..8].copy_from_slice(&(username.len() as u64).to_le_bytes());
    user_data[8..username.len() + 8].copy_from_slice(username.as_bytes());

    let authentication = ClientAuthentication::Unsecure {
        server_addr: server_ip.parse().unwrap(),
        client_id,
        user_data: Some(user_data),
        protocol_id: PROTOCOL_ID,
    };

    let binding = UdpSocket::bind("0.0.0.0:5000");
    if binding.is_err() {
        error!(
            "❌ address already used! Only one client can run on the same machine used as server"
        );
        return;
    }

    let socket = binding.unwrap();
    let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();
    app.insert_resource(transport);

    app.add_systems(Update, handle_connection);

    app.run();
}

fn handle_connection(
    mut client: ResMut<RenetClient>,
    mut transport: ResMut<NetcodeClientTransport>
) {
    client.update(GAME_FPS);
    if transport.update(GAME_FPS, &mut client).is_err() {
        warn!("server is unavailable");
        client.disconnect_due_to_transport();

        // TODO: implementing server shutdown case logic here 👇
        //...
        return;
    }

    if client.is_connected() {
        while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
            if let Ok(event) = deserialize::<GameEvent>(&message) {
                // ! handle server events here
                info!("event from server {:?}", event);
            }
        }
        /*
         * this how to send messages from client to server
         * ex : client.send_message(DefaultChannel::ReliableOrdered, serialize(&event).unwrap());
         */
    }
    transport.send_packets(&mut client).expect("error while sending packets to server");
    sleep(GAME_FPS);
}
