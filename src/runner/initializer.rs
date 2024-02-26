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
        let gameplay = SafeGameDatalayer::new();

        new_matchmaking::backgroung::look_for_matches(&matchmaking);
        new_matchmaking::backgroung::act_on_new_game(
            gameplay.clone(),
            matchmaking.events.on_match_change.clone(),
        );

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
