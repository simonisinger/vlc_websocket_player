
use std::process::exit;
use crossbeam_channel::{Receiver, Sender};
use tokio::net::TcpStream;
use futures_util::{SinkExt, StreamExt};
use futures_util::stream::{SplitSink, SplitStream};
use serde_json::Value;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;

pub struct PlayerWebsocketConnection {}

impl PlayerWebsocketConnection {

    pub async fn send_messages(mut connection: SplitSink<WebSocketStream<TcpStream>, Message>, receiver: Receiver<String>) {
        loop {
            let message = receiver.try_recv();
            if message.is_ok() {
                let result = connection.send(Message::text(message.unwrap())).await;
                if result.is_err() {
                    break;
                }
            }
        }
    }

    pub async fn read_messages(mut connection: SplitStream<WebSocketStream<TcpStream>>, sender: Sender<String>) {
        loop {
            let next = connection.next().await;
            if next.is_none() {
                continue;
            }
            let message_result = next.unwrap();
            if message_result.is_err() {
                continue;
            }
            let msg = message_result.unwrap();
            if msg.is_text() && !msg.is_empty() {
                let raw_json = msg.to_string();
                let json_value: Value = serde_json::from_str(&raw_json).unwrap();
                let command: &str = json_value["command"].as_str().unwrap();
                if command == "close-player" {
                    exit(0);
                }
                let _ = sender.send(msg.to_string());
            }
        }
    }
}
