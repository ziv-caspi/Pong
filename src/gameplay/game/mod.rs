mod ball;
pub mod base;
mod countdown;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
#[derive(Clone)]
pub struct Player {
    id: String,
    position: Position,
}

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
pub struct BallInfo {
    position: Position,
    radius: u8,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
#[derive(Clone)]
pub struct GameState {
    player1_pos: Player,
    player2_pos: Player,
    ball_pos: BallInfo,
    countdown: u8,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
#[derive(Clone)]
pub struct OnGameStateUpdate {
    pub id: String,
    pub state: GameState,
}
