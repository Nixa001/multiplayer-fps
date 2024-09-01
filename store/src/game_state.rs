use crate::*;
use rand::*;
use serde::{ Deserialize, Serialize };
use std::{ collections::HashMap, u8 };

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

/// A GameState object that is able to keep track of game
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GameState {
    pub stage: Stage,
    pub players: HashMap<u8, Players>,
    pub history: Vec<GameEvent>,
    pub id_counter: u8,
    pub lvl: usize,
    pub spawn_positions: Vec<Position>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            stage: Stage::PreGame,
            players: HashMap::new(),
            history: Vec::new(),
            id_counter: 0,
            lvl: 1,
            spawn_positions: Vec::new(),
        }
    }
}

impl GameState {
    /// Determines whether an event is valid considering the current GameState
    pub fn validate(&self, event: &GameEvent, client_id: u64) -> bool {
        match event {
            GameEvent::BeginGame { .. } => {
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

            GameEvent::PlayerJoined { player_id, .. } => {
                // Check that there isn't another player with the same id
                if self.players.contains_key(player_id) || self.stage != Stage::PreGame {
                    return false;
                }
            }
            GameEvent::PlayerDisconnected { player_id } => {
                // Check player exists
                if !self.players.contains_key(player_id) {
                    return false;
                }
            }

            GameEvent::PlayerMove { at: _, .. } => {
                let id = self.get_player_id(client_id);
                if !self.players.contains_key(&id) && id != u8::MAX {
                    return false;
                }
            }
            GameEvent::Spawn { .. } => {
                if self.stage != Stage::PreGame {
                    return false;
                }
            }
            _ => unreachable!(),
        }
        true
    }

    pub fn consume(&mut self, valid_event: &GameEvent, client_id: u64) -> GameEvent {
        let mut eve: GameEvent = GameEvent::BeginGame { player_list: HashMap::new() };
        match valid_event {
            GameEvent::BeginGame { player_list } => {
                self.stage = Stage::InGame;
                eve = GameEvent::BeginGame { player_list: player_list.clone() };
            }

            GameEvent::EndGame => {
                self.stage = Stage::Ended;
                eve = GameEvent::EndGame;
            }

            GameEvent::PlayerJoined { player_id, name, position, client_id } => {
                self.players.insert(*player_id, Players {
                    name: name.to_string(),
                    id: *player_id,
                    position: position.clone(),
                    client_id: client_id.clone(),
                    vision: (0.0, 0.0),
                    lives: 3,
                });

                eve = GameEvent::PlayerJoined {
                    player_id: *player_id,
                    name: name.to_string(),
                    position: position.clone(),
                    client_id: client_id.clone(),
                };
            }

            GameEvent::PlayerDisconnected { player_id } => {
                self.players.remove(player_id);
                eve = GameEvent::PlayerDisconnected { player_id: player_id.clone() };
            }

            GameEvent::PlayerMove { at, vision, .. } => {
                let id = self.get_player_id(client_id);
                let player = self.players.get_mut(&id).unwrap();
                player.position = at.clone();
                player.vision = vision.clone();
                let mut player_list: HashMap<u8, Players> = HashMap::new();
                for (idp, value) in self.players.clone() {
                    if !idp.eq(&id) {
                        player_list.insert(idp, value);
                    }
                }
                eve = GameEvent::PlayerMove {
                    player_id: id,
                    at: at.clone(),
                    player_list,
                    vision: vision.clone(),
                };
            }
            GameEvent::Impact { id } => {
                let impacted_player = self.players.get_mut(id).unwrap();
                impacted_player.lives -= 1;
                return GameEvent::Impact { id: id.clone() };
            },
            GameEvent::Death { player_id } => {
                self.players.remove(player_id);
                return GameEvent::Death { player_id: player_id.clone() };
            }
            _ => {}
        }
        self.history.push(valid_event.clone());
        eve
    }

    pub fn determine_winner(&self) -> Option<u8> {
        if self.players.len() == 1 && self.stage == Stage::InGame {
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
    pub fn set_lvl(&mut self, lvl: usize) {
        self.lvl = lvl;
        self.spawn_positions = get_spawn_positions();
    }

    pub fn random_spawn(&mut self) -> Position {
        let mut rng = thread_rng();
        let gen = rng.gen_range(0..self.spawn_positions.len());
        self.spawn_positions.remove(gen)
    }
    pub fn get_player_id(&self, client_id: u64) -> u8 {
        let mut id: u8 = u8::MAX;
        for (k, v) in &self.players {
            if v.client_id.eq(&client_id) {
                id = k.clone();
                break;
            }
        }
        id
    }

    pub fn get_client_id(&self, id: u8) -> u64 {
        let mut client_id: u64 = u64::MAX;
        for (k, v) in &self.players {
            if k.eq(&id) {
                client_id = v.client_id;
                break;
            }
        }
        client_id
    }
}
