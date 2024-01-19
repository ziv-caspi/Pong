use crate::{
    matchmaking::{queue_up_api::QueueApi, rpc_queue::RpcQueue},
    network::{initializer::Initializer, messages::UserMessage},
};
use anyhow::Result;

use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
};

use super::session::ClientSession;

pub fn start() {
    let user_example = UserMessage::NoUpdates;
    let s = serde_json::to_string(&user_example).unwrap();
    println!("{:?}", s);

    let mut initializer = Initializer::init();

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("started listening on port 7878");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let api = initializer.get_queue_api();
        thread::spawn(|| {
            let mut stream = stream;
            if let Err(_) = handle_connection(&mut stream, api) {
                _ = stream.shutdown(std::net::Shutdown::Both);
                println!("killed client connection")
            }
        });
    }
}

// the protocol is that every loop the client will send a message to the server, it could be a request, or a NoUpdates message.
// NoUpdates message is the servers chance to send push updates.
fn handle_connection(mut stream: &mut TcpStream, api: QueueApi<RpcQueue>) -> Result<()> {
    let mut client_session = ClientSession::new(&api);
    loop {
        let user_message = get_user_message(&mut stream)?;
        let response = client_session.process_message(user_message);
        let json = serde_json::to_string(&response).unwrap();
        stream.write_all(json.as_bytes())?
    }
}

fn get_user_message(stream: &mut TcpStream) -> Result<UserMessage> {
    let mut buf_reader = BufReader::new(stream);
    let mut line = vec![];
    buf_reader.read_until(b'\n', &mut line)?;
    let user_message: UserMessage = serde_json::from_slice(&line)?;
    Ok(user_message)
}
