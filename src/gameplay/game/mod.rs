mod ball;
pub mod base;
mod countdown;
mod score;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, Eq)]

pub struct Player {
    id: String,
    position: Position,
    dimensions: (u32, u32),
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, Eq)]

pub struct Position {
    pub x: u32,
    pub y: u32,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, Eq)]
pub struct BallInfo {
    pub position: Position,
    pub radius: u8,
    pub movement: MovementVector,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, Eq)]
pub struct MovementVector {
    pub horizontal_vector: i32,
    pub vertical_vector: i32,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, Eq)]

pub struct GameState {
    pub player1_pos: Player,
    pub player2_pos: Player,
    pub ball_pos: BallInfo,
    pub countdown: u8,
    pub score: Score,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, Eq)]
pub struct OnGameStateUpdate {
    pub id: String,
    pub state: GameState,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, Eq)]
pub struct Score {
    left_player: PlayerWithScore,
    right_player: PlayerWithScore,
    winner: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, Eq)]
pub struct PlayerWithScore {
    player: Player,
    score: u8,
}
