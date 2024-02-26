use crate::utils::events::EventTopic;

use self::game_datalayer::{GameDatalayer, MemoryGameDatalayer};
use anyhow::Result;
use serde_derive::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

mod game;
pub mod game_datalayer;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
#[derive(Clone)]
pub struct Position {
    x: u32,
    y: u32,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
#[derive(Clone)]
pub struct GameState {
    player1_pos: Position,
    player2_pos: Position,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
#[derive(Clone)]
pub struct OnGameStateUpdate {
    pub id: String,
    pub state: GameState,
}

#[derive(Clone)]
pub struct SafeGameDatalayer {
    inner: Arc<Mutex<MemoryGameDatalayer>>,
    pub on_game_update: EventTopic<OnGameStateUpdate>,
}

impl SafeGameDatalayer {
    pub fn new() -> Self {
        let inner = MemoryGameDatalayer::new();
        let on_game_update = inner.on_game_update.clone();
        Self {
            inner: Arc::new(Mutex::new(inner)),
            on_game_update,
        }
    }
}

impl GameDatalayer for SafeGameDatalayer {
    fn new_game(&mut self, match_id: String, player1_id: String, player2_id: String) -> GameState {
        let mut dl = self.inner.lock().unwrap();
        dl.new_game(match_id, player1_id, player2_id)
    }

    fn move_player(&mut self, match_id: &str, player_id: &str, delta: i32) -> Result<()> {
        let mut dl = self.inner.lock().unwrap();
        dl.move_player(match_id, player_id, delta)
    }

    fn get_on_game_change(&self) -> EventTopic<OnGameStateUpdate> {
        self.on_game_update.clone()
    }
}
