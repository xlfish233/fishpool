mod client;
mod server;

pub mod handler;

use crate::websocket::client::WsClient;
use dashmap::DashMap;
use futures_util::SinkExt;
use salvo::websocket::{Message, WebSocket};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::task;

pub struct CliChan {}
pub struct WsService {
    cid: u64,
    cli_txs: DashMap<u64, Sender<Message>>,
    event_rx: Receiver<Event>,
    event_tx: Sender<Event>,
}

pub enum Event {
    Upgrade(WebSocket),
    Disconnect(u64),
    Message(u64, Message),
}

impl WsService {
    fn on_upgraded(&mut self, ws: WebSocket) {
        let cid = self.cid;
        self.cid += 1;

        let client = WsClient::new(ws, self.event_tx.clone(), cid);

        task::spawn(async move {
            //TODO NEW TASK TO AUTH AND HANDLE INCOMMING MESSAGE.
        });
    }

    async fn on_disconnect(&mut self, uid: u64) {
        if let Some((id, mut tx)) = self.cli_txs.remove(&uid) {
            let _ = tx.send(Message::close()).await;
        }
    }

    fn new() -> Self {
        let (disconnect_tx, disconnect_rx) = tokio::sync::mpsc::channel(1024);
        Self {
            cid: 0,
            cli_txs: DashMap::new(),
            event_rx: disconnect_rx,
            event_tx: disconnect_tx,
        }
    }

    async fn run(&mut self) {
        while let Some(event) = self.event_rx.recv().await {
            match event {
                Event::Upgrade(ws) => {
                    self.on_upgraded(ws);
                }
                Event::Disconnect(uid) => {
                    self.on_disconnect(uid).await;
                }
                Event::Message(uid, msg) => {
                    if let Some(mut tx) = self.cli_txs.get_mut(&uid) {
                        let _ = tx.send(msg).await;
                    }
                }
            }
        }
    }
}
