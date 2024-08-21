use renet::transport::NETCODE_USER_DATA_BYTES;
use std::io::*;
/// Utility function for extracting a player name from renet user data

pub fn name_from_user_data(user_data: &[u8; NETCODE_USER_DATA_BYTES]) -> String {
    let mut buffer = [0u8; 8];
    buffer.copy_from_slice(&user_data[0..8]);
    let mut len = u64::from_le_bytes(buffer) as usize;
    len = len.min(NETCODE_USER_DATA_BYTES - 8);
    let data = user_data[8..len + 8].to_vec();
    String::from_utf8(data).unwrap()
}
pub fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    stdout().flush().unwrap();
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn get_level() -> usize {
    println!("######### MULTIPLAYER-FPS: MAZE WARS #########");
    println!("Welcome warrior !");
    let message = "Pick a level:\n1. lvl 1 (Easy)\n2. lvl 2 (Medium)\n3. lvl 3 (Hard)\n$";
    let mut choice = 0;
    let mut ok = false;
    while !ok {
        let data = get_input(message).parse::<usize>();
        if data.is_err() {
            println!("❌ invalid input, please enter a valid number");
            continue;
        }
        choice = data.unwrap();
        if choice < 1 || choice > 3 {
            println!("❌ invalid input, Please pick a number between 1, 2 & 3 ");
            continue;
        } else {
            ok = true;
        }
    }
    print!("\x1B[2J\x1B[H");
    stdout().flush().unwrap();
    choice
}

pub const PLAYER_LIMIT: usize = 10;
