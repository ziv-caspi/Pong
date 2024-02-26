use std::{thread, time::Duration};

use crate::{
    gameplay::{game_datalayer::GameDatalayer, SafeGameDatalayer},
    utils::events::EventTopic,
};

use super::{datalayer::OnMatchStatusChange, rpc_datalayer::RpcMatchmakingDatalayer};

pub fn look_for_matches(datalayer: &RpcMatchmakingDatalayer) {
    let cloned = datalayer.clone();
    thread::spawn(move || loop {
        _ = cloned.look_for_matches();
        thread::sleep(Duration::from_millis(100))
    });
}

pub fn act_on_new_game(games: SafeGameDatalayer, on_match_change: EventTopic<OnMatchStatusChange>) {
    let mut games = games;
    let on_change = on_match_change.subscribe();
    thread::spawn(move || loop {
        let change = on_change.recv().unwrap();
        if let OnMatchStatusChange::OnStart(start) = change {
            games.new_game(start.match_id, start.player1, start.player2);
        }
    });
}
