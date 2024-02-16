use crate::{
    gameplay::SafeGameDatalayer,
    new_matchmaking::{self, rpc_datalayer::RpcMatchmakingDatalayer},
};

pub struct Initializer {
    matchmaking: RpcMatchmakingDatalayer,
    gameplay: SafeGameDatalayer,
}

impl Initializer {
    pub fn init() -> Initializer {
        let matchmaking = RpcMatchmakingDatalayer::new();
        new_matchmaking::background_match_finder::look_for_matches(&matchmaking);

        let gameplay = SafeGameDatalayer::new();

        Initializer {
            matchmaking,
            gameplay,
        }
    }

    pub fn get_matchmaking(&self) -> RpcMatchmakingDatalayer {
        self.matchmaking.clone()
    }

    pub fn get_gameplay(&self) -> SafeGameDatalayer {
        self.gameplay.clone()
    }
}
