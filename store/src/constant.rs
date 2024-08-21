use std::time::Duration;

const DESIRED_FPS: u64 = 60;
pub const GAME_FPS: Duration = Duration::from_millis(1000 / DESIRED_FPS);
pub const PROTOCOL_ID: u64 = 1582;