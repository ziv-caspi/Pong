use super::{
    ball::{Border, Collision},
    Player, PlayerWithScore, Score,
};

const WINNING_SCORE: u8 = 10;

impl Score {
    pub fn new(left: Player, right: Player) -> Self {
        Score {
            left_player: PlayerWithScore { player: left.id, score: 0 },
            right_player: PlayerWithScore { player: right.id, score: 0 },
            winner: None,
        }
    }

    pub fn update(&mut self, border: &Border) -> bool {
        let mut updated = false;
        if let Border::Left = border {
            self.right_player.score += 1;
            updated = true;
        } else if let Border::Right = border {
            self.left_player.score += 1;
            updated = true;
        }

        // TODO: change score back
        if self.left_player.score >= WINNING_SCORE {
            self.winner = Some(self.left_player.player.clone());
            return true;
        } else if self.right_player.score >= WINNING_SCORE {
            self.winner = Some(self.right_player.player.clone());
            return true;
        }

        updated
    }
}
