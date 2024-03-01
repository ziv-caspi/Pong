use super::Position;

const SCREEN_SIZE: (u32, u32) = (1280, 720);

pub struct Ball {
    pub position: Position,
    pub radius: u8,
    is_down: bool,
    is_right: bool,
}

impl Ball {
    pub fn new() -> Self {
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

    pub fn do_move(&mut self) -> bool {
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
