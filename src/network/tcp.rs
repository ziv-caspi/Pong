use crate::{
    matchmaking::{queue_up_api::QueueApi, rpc_queue::RpcQueue},
    network::{initializer::Initializer, messages::UserMessage},
};
use anyhow::Result;
use std::{
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use super::messages::{QueueUpResponse, ServerMessage};

pub fn start() {
    let user_example = UserMessage::QueueUpRequest(crate::network::messages::QueueUpRequest {
        nickname: String::from("I am ziv"),
    });
    let s = serde_json::to_string(&user_example).unwrap();
    println!("{:?}", s);

    let initializer = Initializer::init();

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
    loop {
        let user_message = get_user_message(&mut stream)?;
        let response = handler_user_message(&api, user_message);
        let json = serde_json::to_string(&response).unwrap();
        stream.write_all(json.as_bytes())?
    }
}

fn handler_user_message(queue_api: &QueueApi<RpcQueue>, message: UserMessage) -> ServerMessage {
    match message {
        UserMessage::QueueUpRequest(request) => {
            let result = match queue_api.register_to_queue(&request.nickname) {
                Ok(user) => Ok(QueueUpResponse { id: user.id }),
                Err(e) => Err(e.to_string()),
            };
            return ServerMessage::QueueUpResponse(result);
        }
        UserMessage::NoUpdates => return ServerMessage::ServerPushUpdate(None), // in the future, this is how I will update on match and gameplay
    };
}

fn get_user_message(stream: &mut TcpStream) -> Result<UserMessage> {
    let mut buf_reader = BufReader::new(stream);
    let mut line = vec![];
    buf_reader.read_until(b'\n', &mut line)?;
    let user_message: UserMessage = serde_json::from_slice(&line)?;
    Ok(user_message)
}
