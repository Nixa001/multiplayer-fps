use serde::{ Deserialize, Serialize };
use crate::*;
/// An event that progresses the GameState forward
#[derive(Debug, Clone, Serialize, PartialEq, Deserialize)]
pub enum GameEvent {
    BeginGame,
    EndGame,
    PlayerJoined {
        player_id: u8,
        name: String,
    },
    PlayerDisconnected {
        player_id: u8,
    },
    PlayerMove {
        player_id: u8,
        at: Position,
    },
    SetId {
        player_id: u8,
    },
}
