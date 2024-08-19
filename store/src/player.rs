use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

/// Struct for storing player related data.
/// In tic-tac-toe the only thing we need is the name and the piece the player will be placing
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub id: u8,
    pub position: Position,
}
