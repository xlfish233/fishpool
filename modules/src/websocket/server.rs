use super::client::{Command, WsClient};
use std::sync::LazyLock;

use dashmap::DashMap;
use salvo::websocket::{Message, WebSocket};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::task;

pub static WS_SERVICE_SENDER: LazyLock<Sender<Event>> = LazyLock::new(|| {
    let mut svr = WsService::new();
    let sender = svr.event_tx.clone();
    tokio::spawn(async move {
        svr.serve().await;
    });
    sender
});

pub struct WsService {
    cid: u64,
    cli_txs: DashMap<u64, Sender<Command>>,
    event_rx: Receiver<Event>,
    event_tx: Sender<Event>,
}

pub enum Event {
    Upgrade(WebSocket),
    Disconnect(u64),
    Message(u64, Message),
    ClientError(u64, anyhow::Error),
}

impl WsService {
    fn on_upgraded(&mut self, ws: WebSocket) {
        let cid = self.cid;
        self.cid += 1;
        let (cmd_tx, cmd_rx) = tokio::sync::mpsc::channel(1024);
        self.cli_txs.insert(cid, cmd_tx);
        let mut client = WsClient::new(ws, self.event_tx.clone(), cid, cmd_rx);
        task::spawn(async move {
            client.serve().await;
        });

        // let mut client = WsClient::new(ws, self.event_tx.clone(), cid);
        // task::spawn(async move {
        //     client.serve().await;
        // });
    }

    async fn on_disconnect(&mut self, uid: u64) {
        if let Some(_) = self.cli_txs.remove(&uid) {}
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
    async fn serve(&mut self) {
        while let Some(e) = self.event_rx.recv().await {
            match e {
                Event::Upgrade(ws) => {
                    self.on_upgraded(ws);
                }
                Event::Disconnect(uid) => {
                    self.on_disconnect(uid).await;
                }
                Event::Message(uid, msg) => {
                    self.on_message(uid, msg).await;
                }
                Event::ClientError(uid, e) => {
                    self.on_client_error(uid, e).await;
                }
            }
        }
    }
    async fn on_message(&mut self, uid: u64, msg: Message) {
        if let Some(tx) = self.cli_txs.get_mut(&uid) {
            //TODO HANDLE THE MESSAGE PROXY TO BACKEND
        }
    }
    async fn on_client_error(&mut self, uid: u64, e: anyhow::Error) {
        // TODO: 处理错误
        if let Some(tx) = self.cli_txs.get_mut(&uid) {}
        tracing::error!("client error: {:?}", e);
    }
}
