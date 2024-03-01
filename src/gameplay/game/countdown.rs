use std::time::Instant;

const COUNTDOWN: u8 = 3;

pub struct Countdown {
    pub current: u8,
    last_change: Instant,
}

impl Countdown {
    pub fn new() -> Self {
        Self {
            current: COUNTDOWN,
            last_change: Instant::now(),
        }
    }

    pub fn count(&mut self) -> bool {
        if self.current == 0 {
            return false;
        }

        let passed = Instant::now() - self.last_change;
        match passed.as_secs() >= 1 {
            true => {
                self.current -= 1;
                self.last_change = Instant::now();
                return true;
            }
            false => false,
        }
    }
}
