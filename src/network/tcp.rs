use std::{
    io::{BufRead, BufReader, Read},
    net::{TcpListener, TcpStream},
    thread,
};

use crate::{
    matchmaking::{queue_up_api::QueueApi, rpc_queue::RpcQueue},
    network::{initializer::Initializer, messages::UserMessage},
};

use super::messages::QueueUpResponse;

pub fn start() {
    let initializer = Initializer::init();

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("started listening on port 7878");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let api = initializer.get_queue_api();
        thread::spawn(|| {
            handle_connection(stream, api);
        });
    }
}

// TODO: make it loop, send response and handle matches
fn handle_connection(mut stream: TcpStream, api: QueueApi<RpcQueue>) {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut line = vec![];
    if let Err(_) = buf_reader.read_until(b'\n', &mut line) {
        println!("error reading full line from client, closing connection");
        _ = stream.shutdown(std::net::Shutdown::Both);
        return;
    }

    let user_message: UserMessage = match serde_json::from_slice(&line) {
        Ok(msg) => msg,
        Err(_) => {
            println!("error parsing json, closing connection");
            _ = stream.shutdown(std::net::Shutdown::Both);
            return;
        }
    };

    match user_message {
        UserMessage::QueueUpRequest(request) => {
            let result = match api.register_to_queue(&request.nickname) {
                Ok(user) => Ok(QueueUpResponse { id: user.id }),
                Err(e) => Err(e.to_string()),
            };
            let json = serde_json::to_string(&result).unwrap();
            // write result to stream
        }
    };

    _ = stream.shutdown(std::net::Shutdown::Both);
}
