use crate::new_matchmaking::{self, rpc_datalayer::RpcMatchmakingDatalayer};

pub struct Initializer {
    matchmaking: RpcMatchmakingDatalayer,
}

impl Initializer {
    pub fn init() -> Initializer {
        let matchmaking = RpcMatchmakingDatalayer::new();
        new_matchmaking::background_match_finder::look_for_matches(&matchmaking);

        Initializer { matchmaking }
    }

    pub fn get_matchmaking(&self) -> RpcMatchmakingDatalayer {
        self.matchmaking.clone()
    }
}
