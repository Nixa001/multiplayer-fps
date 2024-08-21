use crate::*;
use serde::{ Deserialize, Serialize };
/// An event that progresses the GameState forward
#[derive(Debug, Clone, Serialize, PartialEq, Deserialize)]
pub enum GameEvent {
    BeginGame,
    EndGame,
    AccessForbidden,
    PlayerJoined {
        player_id: u8,
        name: String,
        position: Position,
        client_id: u64,
    },
    PlayerDisconnected {
        player_id: u8,
    },
    PlayerMove {
        player_id: u8,
        at: Position,
    },
    Spawn {
        player_id: u8,
        position: Position,
        lvl: usize,
    },
    Timer {
        duration: u8,
    },
}
