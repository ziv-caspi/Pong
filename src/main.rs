mod gameplay;
mod network;
mod new_matchmaking;
mod runner;
mod utils;

fn main() {
    network::websocket::start();
}
