mod matchmaking;
mod network;
mod utils;

fn main() {
    network::tcp::start();
}
