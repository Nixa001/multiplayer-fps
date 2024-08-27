use serde::{ Deserialize, Serialize };

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Position {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

/// Struct for storing player related data.
/// In tic-tac-toe the only thing we need is the name and the piece the player will be placing
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub id: u8,
    pub client_id: u64,
    pub position: Position,
}

impl Player {
    pub fn new(name: String, id: u8, position: Position, client_id: u64) -> Self {
        Self { name, id, position, client_id }
    }
}
