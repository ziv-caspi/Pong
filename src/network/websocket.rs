use crate::{
    gameplay::SafeGameDatalayer,
    new_matchmaking::rpc_datalayer::RpcMatchmakingDatalayer,
    runner::{
        initializer::Initializer,
        messages::{ServerMessage, UserMessage},
        session::ClientSession,
    },
};
use anyhow::{anyhow, bail, Result};
use crossbeam::channel::{bounded, unbounded, Receiver, Sender};
use std::{
    net::TcpStream,
    sync::{Arc, Mutex},
    thread, time::Duration,
};
use websocket::{
    sync::{server::Server, Client, Reader, Writer},
    Message, OwnedMessage,
};

type SharedClientSesssion<'a> = Arc<Mutex<ClientSession<'a, SafeGameDatalayer>>>;

pub fn start() {
    let initializer = Initializer::init();
    let server = Server::bind("0.0.0.0:5000").unwrap();
    println!("websocket server listening on 0.0.0.0:5000");

    for connection in server.filter_map(Result::ok) {
        println!("new client connection");
        let api = initializer.get_matchmaking();
        let gameplay = initializer.get_gameplay();
        // Spawn a new thread for each connection.
        thread::spawn(move || {
            let client = connection.accept().unwrap();
            if let Err(e) = handle_connection(client, api, gameplay) {
                // _ = client.shutdown();
                println!("killed client connection, because: {}", e)
            }
        });
    }
}

fn handle_connection(
    stream: Client<TcpStream>,
    api: RpcMatchmakingDatalayer,
    mut gameplay: SafeGameDatalayer,
) -> Result<()> {
    let client_session = ClientSession::new(&api, &mut gameplay);
    let shared_session = Arc::new(Mutex::new(client_session));
    let (rpc_tx, rpc_rx) = unbounded::<String>();
    let (death_tx, death_rx) = bounded::<()>(1);

    let _ = crossbeam::thread::scope(move |scope| {
        let (mut reader, mut writer) = stream.split().unwrap();
        let reader_shared_session = shared_session.clone();
        let reader_death_tx = death_tx.clone();
        let reader_death_rx = death_rx.clone();
        let writer_death_tx = death_tx.clone();
        let writer_death_rx = death_rx.clone();

        let reader_thread = scope.spawn(move |_| {
            fn read_process(
                mut reader: &mut Reader<TcpStream>,
                shared_session: SharedClientSesssion,
                rpc_tx: Sender<String>,
                death_rx: Receiver<()>,
            ) -> Result<()> {
                if !death_rx.is_empty() {
                    bail!("got death signal");
                }

                let user_message = get_user_message(&mut reader)?;
                let mut session = shared_session.lock().map_err(|e| anyhow!(e.to_string()))?;
                let response = session.process_message(user_message);
                let json = serde_json::to_string(&response).unwrap();
                rpc_tx.send(json)?;
                Ok(())
            }

            loop {
                let read_process = read_process(
                    &mut reader,
                    reader_shared_session.clone(),
                    rpc_tx.clone(),
                    reader_death_rx.clone(),
                );

                if let Err(e) = read_process {
                    println!("couldnt do read process. {}", e);
                    _ = reader.shutdown_all();
                    _ = reader_death_tx.try_send(());
                    break;
                }
            }
        });

        let writer_shared_session = shared_session.clone();
        let writer_thread = scope.spawn(move |_| {
            fn write_process(
                writer: &mut Writer<TcpStream>,
                shared_session: SharedClientSesssion,
                rpc_rx: Receiver<String>,
                death_rx: Receiver<()>,
            ) -> Result<()> {
                if !death_rx.is_empty() {
                    bail!("got death signal");
                }

                if let Ok(json) = rpc_rx.try_recv() {
                    writer.send_message(&Message::text(json))?;
                }

                let mut session = shared_session.lock().map_err(|e| anyhow!(e.to_string()))?;
                let response = session.process_message(UserMessage::NoUpdates);
                let json = serde_json::to_string(&response)?;
                if let ServerMessage::ServerPushUpdate(push) = response {
                    if let Some(_) = push {
                        //thread::sleep(Duration::from_millis(400));
                        writer.send_message(&Message::text(json))?;
                    }
                }

                Ok(())
            }

            loop {
                if let Err(e) = write_process(
                    &mut writer,
                    writer_shared_session.clone(),
                    rpc_rx.clone(),
                    writer_death_rx.clone(),
                ) {
                    println!("couldnt do write process. {}", e);
                    _ = writer.shutdown_all();
                    _ = writer_death_tx.try_send(());
                    break;
                }
            }
        });

        _ = reader_thread.join();
        _ = writer_thread.join();
        shared_session.clone().lock().unwrap().kill_session();
        println!("killed client");
    });

    Ok(())
}

fn get_user_message(stream: &mut Reader<TcpStream>) -> Result<UserMessage> {
    if let OwnedMessage::Text(text) = stream.recv_message()? {
        let user_message: UserMessage = serde_json::from_str(&text)?;
        Ok(user_message)
    } else {
        bail!("message is not text");
    }
}
