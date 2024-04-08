use super::{Player, Position};
const INITIAL_SPEED: u32 = 4;
const MAX__SPEED: u32 = 30;

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
            speed: INITIAL_SPEED,
            screen_size,
        }
    }

    pub fn do_move(&mut self, player1: &Player, player2: &Player) -> bool {
        let (right_player, left_player) = get_right_and_left_players(player1, player2);

        let v_moved = self.vertical_move();
        let h_moved = self.horizontal_move(left_player, right_player);

        h_moved || v_moved
    }

    fn vertical_move(&mut self) -> bool {
        if self.position.y <= 0 + self.radius as u32 {
            self.is_down = true
        } else if self.position.y >= self.screen_size.1 - self.radius as u32 {
            self.is_down = false;
        }

        if self.is_down {
            self.position.y += self.speed;
        } else {
            self.position.y = self.position.y.checked_sub(self.speed).unwrap_or(0);
        }

        true
    }

    fn horizontal_move(&mut self, left_player: &Player, right_player: &Player) -> bool {
        if self.position.x >= self.screen_size.0 - self.radius as u32 {
            self.is_right = false;
        } else if self.position.x <= 0 + self.radius as u32 {
            self.is_right = true;
        }

        if self.collides_with_player(right_player, true)
            || self.collides_with_player(left_player, false)
        {
            self.is_right = !self.is_right;
            // if self.speed <= MAX__SPEED {
            //     self.speed += 2;
            // }
        }

        if self.is_right {
            self.position.x += self.speed;
        } else {
            self.position.x = self.position.x.checked_sub(self.speed).unwrap_or(0);
        }

        true
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
        if is_right {
            horizontal_collision = right >= (player.position.x - player.dimensions.0);
        } else {
            horizontal_collision = left <= (player.position.x + player.dimensions.0);
        }

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
