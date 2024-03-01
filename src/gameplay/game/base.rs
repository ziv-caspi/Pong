use anyhow::{bail, Result};
use std::time::Instant;

use super::{ball::Ball, countdown::Countdown, GameState, Player, Position};

const SCREEN_SIZE: (u32, u32) = (1280, 720);
const PLAYER_START_Y: u32 = SCREEN_SIZE.1 / 2;
const PLAYER_START_X: u32 = SCREEN_SIZE.0 / 15;

const PLAYER1_START_X: u32 = PLAYER_START_X;
const PLAYER2_START_X: u32 = SCREEN_SIZE.0 - PLAYER_START_X;

const SERVER_FPS: u128 = 200;
const MILLIS_BETWEEN_FRAMES: u128 = 1000 / SERVER_FPS;

pub struct Game {
    pub match_id: String,
    player1: Player,
    player2: Player,
    ball: Ball,
    countdown: Countdown,
    last_frame: Instant,
}

impl Game {
    pub fn new(match_id: String, player1: String, player2: String) -> Self {
        Self {
            match_id,
            player1: Player {
                id: player1,
                position: Position {
                    x: PLAYER1_START_X,
                    y: PLAYER_START_Y,
                },
            },
            player2: Player {
                id: player2,
                position: Position {
                    x: PLAYER2_START_X,
                    y: PLAYER_START_Y,
                },
            },
            ball: Ball::new(SCREEN_SIZE),
            countdown: Countdown::new(),
            last_frame: Instant::now(),
        }
    }

    pub fn get_state(&self) -> GameState {
        GameState {
            player1_pos: self.player1.clone(),
            player2_pos: self.player2.clone(),
            ball_pos: super::BallInfo {
                position: self.ball.position.clone(),
                radius: self.ball.radius,
            },
            countdown: self.countdown.current,
        }
    }

    pub fn move_player(&mut self, player_id: &str, delta: u32, up: bool) -> Result<GameState> {
        let player = self.get_player_by_id(player_id)?;

        if up {
            let new_y = player.position.y + delta;
            if new_y <= SCREEN_SIZE.1 {
                player.position.y = new_y;
            } else {
                player.position.y = SCREEN_SIZE.1;
            }
        } else {
            if delta > player.position.y {
                player.position.y = 0;
            } else {
                let new_y = player.position.y - delta;
                player.position.y = new_y;
            }
        }

        Ok(self.get_state())
    }

    pub fn tick(&mut self) -> Option<GameState> {
        let diff = Instant::now() - self.last_frame;
        if diff.as_millis() < MILLIS_BETWEEN_FRAMES {
            return None;
        }
        let count_change = self.countdown.count();
        if count_change {
            return Some(self.get_state());
        }

        if self.countdown.current != 0 {
            return None;
        }

        let ball_change = self.ball.do_move();

        if ball_change {
            return Some(self.get_state());
        } else {
            return None;
        }
    }

    fn get_player_by_id(&mut self, id: &str) -> Result<&mut Player> {
        if &self.player1.id == id {
            return Ok(&mut self.player1);
        } else if &self.player2.id == id {
            return Ok(&mut self.player2);
        } else {
            bail!("could not find player id in this game")
        }
    }
}
