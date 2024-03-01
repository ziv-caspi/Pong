use std::{
    thread,
    time::{Duration, Instant},
};

use anyhow::{bail, Result};

use super::{GameState, Player, Position};

const SCREEN_SIZE: (u32, u32) = (1280, 720);
const PLAYER_START_Y: u32 = SCREEN_SIZE.1 / 2;
const PLAYER_START_X: u32 = SCREEN_SIZE.0 / 15;

const PLAYER1_START_X: u32 = PLAYER_START_X;
const PLAYER2_START_X: u32 = SCREEN_SIZE.0 - PLAYER_START_X;

const COUNTDOWN: u8 = 3;
const SERVER_FPS: u128 = 120;
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
            ball: Ball::new(),
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

struct Countdown {
    current: u8,
    last_change: Instant,
}

impl Countdown {
    fn new() -> Self {
        Self {
            current: COUNTDOWN,
            last_change: Instant::now(),
        }
    }

    fn count(&mut self) -> bool {
        if self.current == 0 {
            return false;
        }

        let passed = Instant::now() - self.last_change;
        match passed.as_secs() >= 1 {
            true => {
                self.current -= 1;
                self.last_change = Instant::now();
                return true;
            }
            false => false,
        }
    }
}

struct Ball {
    position: Position,
    is_down: bool,
    is_right: bool,
    radius: u8,
}

impl Ball {
    fn new() -> Self {
        Self {
            position: Position {
                x: SCREEN_SIZE.0 / 2,
                y: SCREEN_SIZE.1 / 2,
            },
            is_down: true,
            is_right: true,
            radius: 20,
        }
    }

    fn do_move(&mut self) -> bool {
        if self.position.y <= 0 + self.radius as u32 {
            self.is_down = true
        } else if self.position.y >= SCREEN_SIZE.1 - self.radius as u32 {
            self.is_down = false;
        }

        if self.position.x >= SCREEN_SIZE.0 - self.radius as u32 {
            self.is_right = false;
        } else if self.position.x <= 0 + self.radius as u32 {
            self.is_right = true;
        }

        if self.is_down {
            self.position.y += 2;
        } else {
            self.position.y -= 2;
        }

        if self.is_right {
            self.position.x += 2;
        } else {
            self.position.x -= 2;
        }

        true
    }
}
