use super::Position;
const SPEED: u32 = 4;

pub struct Ball {
    pub position: Position,
    pub radius: u32,
    is_down: bool,
    is_right: bool,
    screen_size: (u32, u32),
}

impl Ball {
    pub fn new(screen_size: (u32, u32)) -> Self {
        Self {
            position: Position {
                x: screen_size.0 / 2,
                y: screen_size.1 / 2,
            },
            is_down: true,
            is_right: true,
            radius: 20,
            screen_size,
        }
    }

    pub fn do_move(&mut self, player1: &Position, player2: &Position) -> bool {
        let right_player: &Position;
        let left_player: &Position;
        if player1.x > player2.x {
            right_player = player1;
            left_player = player2;
        } else {
            right_player = player2;
            left_player = player1;
        }

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
            self.position.y += SPEED;
        } else {
            self.position.y -= SPEED;
        }

        true
    }

    fn horizontal_move(&mut self, left_player: &Position, right_player: &Position) -> bool {
        if self.position.x >= self.screen_size.0 - self.radius as u32 {
            self.is_right = false;
        } else if self.position.x <= 0 + self.radius as u32 {
            self.is_right = true;
        }

        let right = self.position.x + self.radius;
        let left = self.position.x - self.radius;
        if right >= right_player.x || left <= left_player.x {
            self.is_right = !self.is_right;
        }

        if self.is_right {
            self.position.x += SPEED;
        } else {
            self.position.x -= SPEED;
        }

        true
    }
}
