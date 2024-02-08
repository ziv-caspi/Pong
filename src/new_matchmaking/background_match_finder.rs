use std::{thread, time::Duration};

use super::rpc_datalayer::RpcMatchmakingDatalayer;

pub fn look_for_matches(datalayer: &RpcMatchmakingDatalayer) {
    let cloned = datalayer.clone();
    thread::spawn(move || loop {
        _ = cloned.look_for_matches();
        thread::sleep(Duration::from_millis(100))
    });
}
