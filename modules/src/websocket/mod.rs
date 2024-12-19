mod client;
mod server;

pub mod handler;

use crate::websocket::client::WsClient;
use salvo::prelude::*;
use salvo::websocket::WebSocket;
use std::collections::HashMap;
use tokio::sync::mpsc::Receiver;

struct WsService {
    socks: HashMap<u64, WsClient>,
    disconnect_ch: Receiver<u64>,
}

impl WsService {
    fn on_upgraded(&self, ws: WebSocket) {
        //TODO CREATE WS CLIENT AND PUT IT IN SOCKS
    }
    async fn on_disconnect(&mut self, uid: u64) {
        let cli = self.socks.get_mut(&uid).unwrap();
        cli.close().await;
        self.socks.remove(&uid);
    }
}
