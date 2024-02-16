use self::game_datalayer::{GameDatalayer, MemoryGameDatalayer};
use anyhow::Result;
use std::sync::{Arc, Mutex};

mod game;
pub mod game_datalayer;

#[derive(Clone)]
pub struct Position {
    x: u32,
    y: u32,
}

#[derive(Clone)]
pub struct GameState {
    player1_pos: Position,
    player2_pos: Position,
}

#[derive(Clone)]
pub struct OnGameStateUpdate {
    id: String,
    state: GameState,
}

pub struct SafeGameDatalayer {
    inner: Arc<Mutex<MemoryGameDatalayer>>,
}

impl SafeGameDatalayer {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(MemoryGameDatalayer::new())),
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
}
