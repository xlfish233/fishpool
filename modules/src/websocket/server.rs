use super::client::{Command, WsClient};
use super::handler_ext::HandlerExt;
use anyhow::Error;
use async_trait::async_trait;
use dashmap::DashMap;
use salvo::websocket::{Message, WebSocket};
use std::sync::LazyLock;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::task;

pub static WS_SERVICE_SENDER: LazyLock<Sender<Event>> = LazyLock::new(|| {
    let mut svr = WsService::<HandlerExt>::new();
    let sender = svr.event_tx.clone();
    tokio::spawn(async move {
        svr.serve().await;
    });
    sender
});
#[async_trait]
pub trait WsEventHandler: Send + Sync {
    async fn handle_upgraded_ws(&self, ws: WebSocket);
    async fn handle_message(&self, client_id: u64, msg: Message);
    async fn handle_disconnect(&self, client_id: u64);
    async fn handle_client_error(&self, client_id: u64, err: Error);
    fn new() -> Self
    where
        Self: Sized;
}

pub struct WsService<H>
where
    H: WsEventHandler,
{
    cid: u64,
    cli_txs: DashMap<u64, Sender<Command>>,
    event_rx: Receiver<Event>,
    event_tx: Sender<Event>,
    handler: H,
}

pub enum Event {
    Upgrade(WebSocket),
    Disconnect(u64),
    Message(u64, Message),
    ClientError(u64, Error),
}

impl<H: WsEventHandler> WsService<H> {
    fn on_upgraded(&mut self, ws: WebSocket) {
        let cid = self.cid;
        self.cid += 1;
        let (cmd_tx, cmd_rx) = tokio::sync::mpsc::channel(1024);
        self.cli_txs.insert(cid, cmd_tx);
        let mut client = WsClient::new(ws, self.event_tx.clone(), cid, cmd_rx);
        task::spawn(async move {
            client.serve().await;
        });
    }

    async fn on_disconnect(&mut self, uid: u64) {
        self.cli_txs.remove(&uid);
    }

    fn new() -> Self {
        let (disconnect_tx, disconnect_rx) = tokio::sync::mpsc::channel(1024);
        Self {
            cid: 0,
            cli_txs: DashMap::new(),
            event_rx: disconnect_rx,
            event_tx: disconnect_tx,
            handler: H::new(),
        }
    }
    async fn serve(&mut self) {
        while let Some(e) = self.event_rx.recv().await {
            match e {
                Event::Upgrade(ws) => {

                    self.handler.handle_upgraded_ws(ws).await;
                }
                Event::Disconnect(uid) => {
                    self.cli_txs.remove(&uid);
                    self.handler.handle_disconnect(uid).await;
                }

                Event::Message(uid, msg) => {
                    self.handler.handle_message(uid, msg).await;
                }
                Event::ClientError(uid, e) => {
                    self.handler.handle_client_error(uid, e).await;
                }
            }
        }
    }
}
