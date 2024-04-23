use crate::utils::events::EventTopic;

use self::{
    game::{GameState, OnGameStateUpdate},
    game_datalayer::MemoryGameDatalayer,
};
use anyhow::Result;
use std::sync::{Arc, Mutex};

pub mod background;
pub mod game;
pub mod game_datalayer;

pub trait GameDatalayer {
    fn get_on_game_change(&self) -> EventTopic<OnGameStateUpdate>;
    fn new_game(&mut self, match_id: String, player1_id: String, player2_id: String) -> GameState;
    fn move_player(
        &mut self,
        match_id: &str,
        player_id: &str,
        delta: i32,
        action_id: &str,
    ) -> Result<()>;
    fn tick(&mut self);
    fn remove_player(&mut self, player: &str, game: &str) -> Result<()>;
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
        Self { inner: Arc::new(Mutex::new(inner)), on_game_update }
    }
}

impl GameDatalayer for SafeGameDatalayer {
    fn new_game(&mut self, match_id: String, player1_id: String, player2_id: String) -> GameState {
        let mut dl = self.inner.lock().unwrap();
        dl.new_game(match_id, player1_id, player2_id)
    }

    fn move_player(
        &mut self,
        match_id: &str,
        player_id: &str,
        delta: i32,
        action_id: &str,
    ) -> Result<()> {
        let mut dl = self.inner.lock().unwrap();
        dl.move_player(match_id, player_id, delta, action_id)
    }

    fn get_on_game_change(&self) -> EventTopic<OnGameStateUpdate> {
        self.on_game_update.clone()
    }

    fn tick(&mut self) {
        let mut dl = self.inner.lock().unwrap();
        dl.tick()
    }

    fn remove_player(&mut self, player: &str, game: &str) -> Result<()> {
        let mut dl = self.inner.lock().unwrap();
        dl.remove_player(player, game)
    }
}
