mod client;
mod server;

pub mod handler;

use crate::websocket::client::WsClient;
use salvo::websocket::WebSocket;
use std::collections::HashMap;
use tokio::sync::mpsc::{Receiver, Sender};

struct WsService {
    cid: u64,
    socks: HashMap<u64, WsClient>,
    disconnect_rx: Receiver<u64>,
    disconnect_tx: Sender<u64>,
}

impl WsService {
    fn on_upgraded(&mut self, ws: WebSocket) {
        //TODO CREATE WS CLIENT AND PUT IT IN SOCKS
        let cid = self.cid;
        self.cid += 1;
        WsClient::new(ws, self.disconnect_tx.clone(), cid);
    }
    async fn on_disconnect(&mut self, uid: u64) {
        let cli = self.socks.get_mut(&uid).unwrap();
        cli.close().await;
        self.socks.remove(&uid);
    }
    fn new() -> Self {
        let (disconnect_tx, disconnect_rx) = tokio::sync::mpsc::channel(1024);
        Self {
            cid: 0,
            socks: HashMap::new(),
            disconnect_rx,
            disconnect_tx,
        }
    }
}
