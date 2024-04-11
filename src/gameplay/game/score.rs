use super::{
    ball::{Border, Collision},
    Player, PlayerWithScore, Score,
};

impl Score {
    pub fn new(left: Player, right: Player) -> Self {
        Score {
            left_player: PlayerWithScore { player: left, score: 0 },
            right_player: PlayerWithScore { player: right, score: 0 },
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

        if self.left_player.score >= 3 {
            self.winner = Some(self.left_player.player.id.to_owned());
            return true;
        } else if self.right_player.score >= 3 {
            self.winner = Some(self.right_player.player.id.to_owned());
            return true;
        }

        updated
    }
}
