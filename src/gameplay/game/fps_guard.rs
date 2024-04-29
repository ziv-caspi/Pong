use std::time::Instant;

pub type FpsReport = u128;
pub enum FpsResult {
    Run(Option<FpsReport>),
    Skip,
}

pub struct FpsGuard {
    desired_fps: u8,
    last_frame: Instant,
    last_second_start: Instant,
    last_seconds_frames: u128,
}

impl FpsGuard {
    pub fn new(desired_fps: u8) -> Self {
        FpsGuard {
            desired_fps,
            last_frame: Instant::now(),
            last_second_start: Instant::now(),
            last_seconds_frames: 0,
        }
    }

    pub fn guard(&mut self) -> FpsResult {
        let diff = self.last_frame.elapsed().as_millis();
        if diff < self.millis_between_frames() {
            return FpsResult::Skip;
        } else {
            self.last_frame += self.last_frame.elapsed();
        }

        let mut result = FpsResult::Run(None);
        self.last_seconds_frames += 1;
        if self.last_second_start.elapsed().as_millis() >= 1000 {
            result = FpsResult::Run(Some(self.last_seconds_frames));
            self.last_seconds_frames = 0;
            self.last_second_start += self.last_second_start.elapsed();
        }

        result
    }

    fn millis_between_frames(&self) -> u128 {
        1000 / self.desired_fps as u128
    }
}
