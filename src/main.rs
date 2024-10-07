mod player_websocket_connection;
mod player;

extern crate vlc;

use std::thread;
use tokio::net::TcpListener;
use crossbeam_channel::{Receiver, Sender, unbounded};
use futures_util::StreamExt;

#[tokio::main]
async fn main() {
    let (sender_player, receiver_websocket): (Sender<String>, Receiver<String>) = unbounded();
    let (sender_websocket, receiver_player): (Sender<String>, Receiver<String>) = unbounded();
    
    tokio::spawn(async move {
        let mut player = player::Player::new(sender_player, receiver_player);
        player.process_messages();
    });

    let server = TcpListener::bind("127.0.0.1:5000").await.unwrap();
    while let Ok((stream, _)) = server.accept().await {
        let sender_local = sender_websocket.clone();
        let receiver_local = receiver_websocket.clone();
        let ws_stream = tokio_tungstenite::accept_async(stream).await.unwrap();
        let (write, read) = ws_stream.split();
        thread::spawn(move || {
            futures::executor::block_on(player_websocket_connection::PlayerWebsocketConnection::read_messages(read, sender_local));
        });
        thread::spawn(move || {
            futures::executor::block_on(player_websocket_connection::PlayerWebsocketConnection::send_messages(write, receiver_local));
        });
    }
}