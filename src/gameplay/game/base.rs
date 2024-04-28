use anyhow::{bail, Result};
use crossbeam::channel::{unbounded, Receiver, Sender};

use super::{
    ball::{Ball, BallMovementResult, Collision},
    countdown::Countdown,
    fps_guard::{FpsGuard, FpsResult},
    GameState, Player, Position, Score,
};

const SCREEN_SIZE: (u32, u32) = (667, 300);
const PLAYER_SIZE: (u32, u32) = (5, 75);
const PLAYER_START_Y: u32 = SCREEN_SIZE.1 / 2;
const PLAYER_START_X: u32 = SCREEN_SIZE.0 / 8;

const PLAYER1_START_X: u32 = PLAYER_START_X;
const PLAYER2_START_X: u32 = SCREEN_SIZE.0 - PLAYER_START_X;

const SERVER_FPS: u8 = 60;

struct MovePlayerCommand {
    player_id: String,
    delta: u32,
    up: bool,
    action_id: String
}

pub struct Game {
    pub match_id: String,
    player1: Player,
    player2: Player,
    ball: Ball,
    countdown: Countdown,
    score: Score,
    fps_guard: FpsGuard,
    recent_actions: Vec<String>, // TODO: can be heavily optimized
    pending_actions_reciever: Receiver<MovePlayerCommand>,
    pending_actions_sender: Sender<MovePlayerCommand>,
}

impl Game {
    pub fn new(match_id: String, player1: String, player2: String) -> Self {
        let player1 = Player {
            id: player1,
            position: Position { x: PLAYER1_START_X, y: PLAYER_START_Y },
            dimensions: PLAYER_SIZE,
        };
        let player2 = Player {
            id: player2,
            position: Position { x: PLAYER2_START_X, y: PLAYER_START_Y },
            dimensions: PLAYER_SIZE,
        };
        let (tx, rx) = unbounded();
        Self {
            score: Score::new(player1.clone(), player2.clone()),
            match_id,
            player1,
            player2,
            ball: Ball::new(SCREEN_SIZE),
            countdown: Countdown::new(),
            fps_guard: FpsGuard::new(SERVER_FPS),
            recent_actions: vec![],
            pending_actions_reciever: rx,
            pending_actions_sender: tx,
        }
    }

    pub fn get_state(&mut self) -> GameState {
        let mut horizontal_vector = self.ball.speed as i32;
        let mut vertical_vector = self.ball.speed as i32;
        if !self.ball.is_right {
            horizontal_vector *= -1;
        }
        if !self.ball.is_down {
            vertical_vector *= -1;
        }

        let state = GameState {
            player1_pos: self.player1.clone(),
            player2_pos: self.player2.clone(),
            ball_pos: super::BallInfo {
                position: self.ball.position.clone(),
                radius: self.ball.radius as u8,
                movement: super::MovementVector { horizontal_vector, vertical_vector },
            },
            countdown: self.countdown.current,
            score: self.score.clone(),
            recent_handled_actions: self.recent_actions.clone(),
        };

        if self.recent_actions.len() > 1500 {
            self.recent_actions.drain(0..300);
        }
         
        state
    }

    pub fn move_player(&mut self,
        player_id: &str,
        delta: u32,
        up: bool,
        action_id: &str) {
        _ = self.pending_actions_sender.send(MovePlayerCommand {
            player_id: player_id.to_owned(),
            delta, 
            up, 
            action_id: action_id.to_owned()
        });
    }

    fn inner_move_player(
        &mut self,
        player_id: &str,
        delta: u32,
        up: bool,
        action_id: &str,
    ) -> Result<()> {
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

        self.recent_actions.push(action_id.to_owned());
        Ok(())
    }

    pub fn tick(&mut self) -> Option<GameState> {
        match self.fps_guard.guard() {
            FpsResult::Run(report) => {
                if let Some(fps) = report {
                    println!("FPS: {}", fps);
                }
            }
            FpsResult::Skip => return None,
        }

        let to_read = self.pending_actions_reciever.len();
        let mut read: usize = 0;
        loop {
            if read >= to_read {
                break;
            }

            if let Ok(msg) = self.pending_actions_reciever.try_recv() {
                _ = self.inner_move_player(&msg.player_id, msg.delta, msg.up, &msg.action_id);
                read += 1;
            } else {
                break;
            }
        }


        let count_change = self.countdown.count();
        if count_change {
            return Some(self.get_state());
        }

        if self.countdown.current != 0 {
            return None;
        }

        let ball_change = self.ball.do_move(&self.player1, &self.player2);
        if let BallMovementResult::MoveCollide(Collision::BorderCollision(border)) = &ball_change {
            if self.score.update(border) {
                if let Some(_) = &self.score.winner {
                    self.ball.respawn();
                } else {
                    //self.ball.respawn();
                    //self.countdown.after_score();
                }
            }
        }

        if let BallMovementResult::Move = ball_change {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fpss() {
        let mut game = Game::new(String::from("ABC"), String::from("1"), String::from("2"));
        loop {
            game.tick();
        }
    }
}
