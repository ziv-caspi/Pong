use std::time::Instant;

pub type FpsReport = u128;
pub type SimulatedRuns = u8;
pub enum FpsResult {
    Run(SimulatedRuns, Option<FpsReport>),
    Skip,
}

pub struct FpsGuard {
    simulated_desired_fps: u8,
    actual_desired_fps: u8,
    last_frame: Instant,
    last_second_start: Instant,
    last_seconds_frames: u128,
}

impl FpsGuard {
    pub fn new(actual_desired_fps: u8, simulated_desired_fps: u8) -> Self {
        FpsGuard {
            simulated_desired_fps,
            actual_desired_fps,
            last_frame: Instant::now(),
            last_second_start: Instant::now(),
            last_seconds_frames: 0,
        }
    }

    pub fn guard(&mut self) -> FpsResult {
        let diff = self.last_frame.elapsed().as_millis();
        if diff < self.millis_between_frames() {
            return FpsResult::Skip;
        }

        self.last_frame += self.last_frame.elapsed();
        let simulated_runs = self.simulated_desired_fps / self.actual_desired_fps;

        let mut result = FpsResult::Run(simulated_runs, None);
        self.last_seconds_frames += 1;
        if self.last_second_start.elapsed().as_millis() >= 1000 {
            result = FpsResult::Run(simulated_runs, Some(self.last_seconds_frames));
            self.last_seconds_frames = 0;
            self.last_second_start += self.last_second_start.elapsed();
        }

        result
    }

    fn millis_between_frames(&self) -> u128 {
        1000 / self.actual_desired_fps as u128
    }
}
