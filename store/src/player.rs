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
pub struct Players {
    pub name: String,
    pub id: u8,
    pub client_id: u64,
    pub position: Position,
    pub vision: (f32, f32),
    pub lives: u8,
}

impl Players {
    pub fn new(
        name: String,
        id: u8,
        position: Position,
        vision: (f32, f32),
        client_id: u64,
    ) -> Self {
        Self { name, id, position, vision, client_id, lives:3 }
    }
}
