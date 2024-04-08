use super::{game::base::Game, GameDatalayer, GameState, OnGameStateUpdate};
use crate::utils::events::EventTopic;
use anyhow::{anyhow, bail, Result};

pub struct MemoryGameDatalayer {
    games: Vec<Game>,
    pub on_game_update: EventTopic<OnGameStateUpdate>,
}

impl MemoryGameDatalayer {
    pub fn new() -> Self {
        Self { games: vec![], on_game_update: EventTopic::new() }
    }

    fn get_game_by_id(&mut self, id: &str) -> Result<&mut Game> {
        for game in &mut self.games {
            if game.match_id == id {
                return Ok(game);
            }
        }

        bail!("could not find this game id")
    }
}

impl GameDatalayer for MemoryGameDatalayer {
    fn new_game(&mut self, match_id: String, player1_id: String, player2_id: String) -> GameState {
        let game = Game::new(match_id.clone(), player1_id, player2_id);
        let state = game.get_state();
        self.games.push(game);
        self.on_game_update.invoke(OnGameStateUpdate { id: match_id, state: state.clone() });
        state
    }

    fn move_player(&mut self, match_id: &str, player_id: &str, delta: i32) -> Result<()> {
        let game = self.get_game_by_id(match_id)?;

        let normalized: u32;
        let up: bool;
        if delta >= 0 {
            normalized = delta as u32;
            up = true;
        } else {
            normalized = (delta * -1) as u32;
            up = false;
        }

        let state = game.move_player(player_id, normalized, up)?;
        self.on_game_update.invoke(OnGameStateUpdate { id: match_id.to_owned(), state: state });
        Ok(())
    }

    fn get_on_game_change(&self) -> EventTopic<OnGameStateUpdate> {
        self.on_game_update.clone()
    }

    fn tick(&mut self) {
        for game in &mut self.games {
            if let Some(state) = game.tick() {
                self.on_game_update
                    .invoke(OnGameStateUpdate { id: game.match_id.clone(), state: state })
            }
        }
    }

    fn remove_player(&mut self, player: &str, game: &str) -> Result<()> {
        let position = match self.games.iter().position(|g| g.match_id == game) {
            Some(pos) => Ok(pos),
            None => Err(anyhow!("could not find player with this id")),
        }?;
        self.games.remove(position);
        Ok(())
    }
}
