use crate::{
    gameplay::{self, SafeGameDatalayer},
    new_matchmaking::{self, rpc_datalayer::RpcMatchmakingDatalayer},
    runner::session::ClientSession,
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
        gameplay::background::run_ticker(gameplay.clone());

        Initializer { matchmaking, gameplay }
    }

    pub fn get_matchmaking(&self) -> RpcMatchmakingDatalayer {
        self.matchmaking.clone()
    }

    pub fn get_gameplay(&self) -> SafeGameDatalayer {
        self.gameplay.clone()
    }
}

// pub struct Initializer2 {
//     container: ioc::Container,
// }

// impl Initializer2 {
//     pub fn new() -> Initializer2 {
//         let mut container = ioc::Container::new();

//         container.register_singleton("matchmaking", |_| RpcMatchmakingDatalayer::new());
//         container.register_singleton("game", |_| SafeGameDatalayer::new());

//         container.register_scoped("client", |container| {
//             let matchmaking: RpcMatchmakingDatalayer =
//                 container.resolve_singelton("matchmaking").unwrap();
//             let mut gameplay = container.resolve_singelton::<SafeGameDatalayer>("game").unwrap();
//             Box::new(ClientSession::new(&matchmaking, &mut gameplay))
//         });

//         todo!()
//     }
// }
