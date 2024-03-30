use crate::{
    gameplay::SafeGameDatalayer,
    new_matchmaking::rpc_datalayer::RpcMatchmakingDatalayer,
    runner::{initializer::Initializer, messages::UserMessage, session::ClientSession},
};
use anyhow::{bail, Ok, Result};
use std::{net::TcpStream, thread};
use websocket::{
    sync::{server::Server, Client},
    Message, OwnedMessage,
};

pub fn start() {
    let initializer = Initializer::init();
    let server = Server::bind("0.0.0.0:5000").unwrap();
    println!("websocket server listening on 0.0.0.0:5000");

    for connection in server.filter_map(Result::ok) {
        let api = initializer.get_matchmaking();
        let mut gameplay = initializer.get_gameplay();
        // Spawn a new thread for each connection.
        thread::spawn(move || {
            let mut client = connection.accept().unwrap();
            if let Err(e) = handle_connection(&mut client, &api, &mut gameplay) {
                _ = client.shutdown();
                println!("killed client connection, because: {}", e)
            }
        });
    }
}

// the protocol is that every loop the client will send a message to the server, it could be a request, or a NoUpdates message.
// NoUpdates message is the servers chance to send push updates.
fn handle_connection(
    stream: &mut Client<TcpStream>,
    api: &RpcMatchmakingDatalayer,
    gameplay: &mut SafeGameDatalayer,
) -> Result<()> {
    let mut client_session = ClientSession::new(api, gameplay);
    loop {
        if let Err(e) = protocol_cycle(stream, &mut client_session) {
            println!("couldnt execute protocol cycle. err: {}", e);
            client_session.kill_session();
            return Err(e);
        }
    }
}

fn protocol_cycle(
    stream: &mut Client<TcpStream>,
    client_session: &mut ClientSession<'_, SafeGameDatalayer>,
) -> Result<()> {
    let user_message = get_user_message(stream)?;
    let response = client_session.process_message(user_message);
    let json = serde_json::to_string(&response)?;
    let message = Message::text(json);
    stream.send_message(&message)?;
    Ok(())
}

fn get_user_message(stream: &mut Client<TcpStream>) -> Result<UserMessage> {
    if let OwnedMessage::Text(text) = stream.recv_message()? {
        let user_message: UserMessage = serde_json::from_str(&text)?;
        Ok(user_message)
    } else {
        bail!("message is not text");
    }
}
