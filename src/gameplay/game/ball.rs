use std::cmp::Ordering;

use rand::{rngs::ThreadRng, Rng};

use super::{Player, Position};
const INITIAL_SPEED: u32 = 4;
const MAX_SPEED: u32 = 30;

#[derive(Eq, PartialEq, PartialOrd, Ord)]
pub enum BallMovementResult {
    NoMove,
    Move,
    MoveCollide(Collision),
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Collision {
    PlayerCollision(String),
    BorderCollision(Border),
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Border {
    Top,
    Bottom,
    Left,
    Right,
}

pub struct Ball {
    pub position: Position,
    pub radius: u32,
    pub is_down: bool,
    pub is_right: bool,
    pub speed: u32,
    screen_size: (u32, u32),
}

impl Ball {
    pub fn new(screen_size: (u32, u32)) -> Self {
        Self {
            position: Position { x: screen_size.0 / 2, y: screen_size.1 / 2 },
            is_down: true,
            is_right: true,
            radius: 8,
            speed: 0,
            screen_size,
        }
    }

    pub fn do_move(&mut self, player1: &Player, player2: &Player) -> BallMovementResult {
        // handle start after countdown
        if self.speed == 0 {
            self.speed = INITIAL_SPEED;
        }

        let (right_player, left_player) = get_right_and_left_players(player1, player2);

        let v_res = self.vertical_move();
        let h_res = self.horizontal_move(left_player, right_player);

        if v_res > h_res {
            return v_res;
        } else {
            return h_res;
        }
    }

    pub fn respawn(&mut self) {
        let mut rand = rand::thread_rng();
        self.position.x = self.screen_size.0 / 2;
        let y = rand.gen_range(100, self.screen_size.1 - 100);
        self.position.y = y;
        let right = rand.gen_bool(0.5);
        self.is_right = right;
    }

    fn vertical_move(&mut self) -> BallMovementResult {
        let mut border_col: Option<Collision> = None;
        if self.position.y <= 0 + self.radius as u32 {
            self.is_down = true;
            border_col = Some(Collision::BorderCollision(Border::Top));
        } else if self.position.y >= self.screen_size.1 - self.radius as u32 {
            self.is_down = false;
            border_col = Some(Collision::BorderCollision(Border::Bottom));
        }

        if self.is_down {
            self.position.y += self.speed;
        } else {
            self.position.y = self.position.y.checked_sub(self.speed).unwrap_or(0);
        }

        match border_col {
            Some(col) => BallMovementResult::MoveCollide(col),
            None => BallMovementResult::Move,
        }
    }

    fn horizontal_move(
        &mut self,
        left_player: &Player,
        right_player: &Player,
    ) -> BallMovementResult {
        let mut border_col: Option<Collision> = None;
        if self.position.x >= self.screen_size.0 - self.radius as u32 {
            self.is_right = false;
            border_col = Some(Collision::BorderCollision(Border::Right));
        } else if self.position.x <= 0 + self.radius as u32 {
            self.is_right = true;
            border_col = Some(Collision::BorderCollision(Border::Left));
        }

        let right_player_col = self.collides_with_player(right_player, true);
        let left_player_col = self.collides_with_player(left_player, false);
        if right_player_col || left_player_col {
            self.is_right = !self.is_right;
            // if self.speed <= MAX__SPEED {
            //     self.speed += 2;
            // }
            let player: &str;
            if right_player_col {
                player = &right_player.id;
            } else {
                player = &left_player.id;
            }
            return BallMovementResult::MoveCollide(Collision::PlayerCollision(player.to_owned()));
        }

        if self.is_right {
            self.position.x += self.speed;
        } else {
            self.position.x = self.position.x.checked_sub(self.speed).unwrap_or(0);
        }

        match border_col {
            Some(col) => BallMovementResult::MoveCollide(col),
            None => BallMovementResult::Move,
        }
    }

    fn collides_with_player(&self, player: &Player, is_right: bool) -> bool {
        if is_right && !self.is_right {
            return false;
        }
        if !is_right && self.is_right {
            return false;
        }

        let right = self.position.x + self.radius;
        let left = self.position.x.checked_sub(self.radius).unwrap_or(0);
        let top = self.position.y.checked_sub(self.radius).unwrap_or(0);
        let bottom = self.position.y + self.radius;

        let horizontal_collision: bool;
        let player_horizontal = player.position.x..player.position.x + player.dimensions.0;
        horizontal_collision =
            player_horizontal.contains(&right) || player_horizontal.contains(&left);

        //debug

        if is_right {
            if right >= (player.position.x - player.dimensions.0) {
                let x = 5;
            }
        } else {
            if left <= (player.position.x + player.dimensions.0) {
                let x = 5;
            }
        }

        //return false;
        // BUG: when going past player but entering its height, the collision happens before border
        horizontal_collision
            && bottom >= player.position.y
            && top <= player.position.y + player.dimensions.1
    }
}

fn get_right_and_left_players<'a>(
    player1: &'a Player,
    player2: &'a Player,
) -> (&'a Player, &'a Player) {
    let right_player: &Player;
    let left_player: &Player;
    if player1.position.x > player2.position.x {
        right_player = player1;
        left_player = player2;
    } else {
        right_player = player2;
        left_player = player1;
    }
    (right_player, left_player)
}
