use super::{GameDatalayer, SafeGameDatalayer};
use std::{thread, time::Duration};

pub fn run_ticker(mut game_dl: SafeGameDatalayer) {
    thread::spawn(move || loop {
        game_dl.tick();
        thread::sleep(Duration::from_millis(1));
    });
}
