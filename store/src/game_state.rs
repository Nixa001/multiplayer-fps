use serde::{ Deserialize, Serialize };
use std::collections::HashMap;
use crate::*;

/// The different states a game can be in. (not to be confused with the entire "GameState")
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Stage {
    PreGame,
    InGame,
    Ended,
}
/// The reasons why a game could end
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Deserialize)]
pub enum EndGameReason {
    PlayerWon {
        winner: u64,
    },
}

/// A GameState object that is able to keep track of a game of TicTacTussle
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GameState {
    pub stage: Stage,
    pub players: HashMap<u8, Player>,
    pub history: Vec<GameEvent>,
    pub id_counter: u8,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            stage: Stage::PreGame,
            players: HashMap::new(),
            history: Vec::new(),
            id_counter: 0,
        }
    }
}

impl GameState {
    /// Determines whether an event is valid considering the current GameState
    pub fn validate(&self, event: &GameEvent) -> bool {
        match event {
            GameEvent::BeginGame => {
                // Check that the game hasn't started yet. (we don't want to double start a game)
                if self.stage != Stage::PreGame {
                    return false;
                }
            }

            GameEvent::EndGame => {
                // Check that the game has started before someone wins it
                if self.stage != Stage::InGame {
                    return false;
                }
            }

            GameEvent::PlayerJoined { player_id, name: _ } => {
                // Check that there isn't another player with the same id
                if self.players.contains_key(player_id) {
                    return false;
                }
            }
            GameEvent::PlayerDisconnected { player_id } => {
                // Check player exists
                if !self.players.contains_key(player_id) {
                    return false;
                }
            }

            GameEvent::PlayerMove { player_id, at: _ } => {
                if !self.players.contains_key(player_id) {
                    return false;
                }
            }
            GameEvent::SetId { player_id: _ } => {
                if self.stage != Stage::PreGame {
                    return false;
                }
            }
        }
        true
    }

    pub fn consume(&mut self, valid_event: &GameEvent) {
        match valid_event {
            GameEvent::BeginGame => {
                self.stage = Stage::InGame;
            }

            GameEvent::EndGame => {
                self.stage = Stage::Ended;
            }

            GameEvent::PlayerJoined { player_id, name } => {
                // ! updated and define position here
                self.players.insert(*player_id, Player {
                    name: name.to_string(),
                    id: *player_id,
                    position: Position::default(),
                });
            }

            GameEvent::PlayerDisconnected { player_id } => {
                self.players.remove(player_id);
            }

            GameEvent::PlayerMove { player_id, at } => {
                // ! must check this part for coming features
                let player = self.players.get_mut(player_id).unwrap();
                player.position = at.clone();
            }
            _ => {}
        }

        self.history.push(valid_event.clone());
    }

    pub fn determine_winner(&self) -> Option<u8> {
        if self.players.len() == 1 {
            for (id, _) in &self.players {
                return Some(*id);
            }
        }
        None
    }

    pub fn generate_id(&mut self) -> u8 {
        let id = self.id_counter;
        self.id_counter += 1;
        id
    }
}
