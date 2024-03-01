use super::Position;
const SPEED: u32 = 4;

pub struct Ball {
    pub position: Position,
    pub radius: u8,
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

    pub fn do_move(&mut self) -> bool {
        if self.position.y <= 0 + self.radius as u32 {
            self.is_down = true
        } else if self.position.y >= self.screen_size.1 - self.radius as u32 {
            self.is_down = false;
        }

        if self.position.x >= self.screen_size.0 - self.radius as u32 {
            self.is_right = false;
        } else if self.position.x <= 0 + self.radius as u32 {
            self.is_right = true;
        }

        if self.is_down {
            self.position.y += SPEED;
        } else {
            self.position.y -= SPEED;
        }

        if self.is_right {
            self.position.x += SPEED;
        } else {
            self.position.x -= SPEED;
        }

        true
    }
}
