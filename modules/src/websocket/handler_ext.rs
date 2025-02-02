use super::server::{Event, WsEventHandler};
use anyhow::Error;
use salvo::prelude::*;
use salvo::websocket::Message;

use tokio::sync::mpsc::Sender;
use tracing::debug;

pub struct HandlerExt {}

#[async_trait]
impl WsEventHandler for HandlerExt {
    async fn handle_upgraded_ws(&self, cid: u64,_svr_tx: Sender<Event>) {
        debug!("Client {} connected", cid);
    }

    async fn handle_message(&self, client_id: u64, msg: Message, _svr_tx: Sender<Event>) {
        debug!("Client {} sent message: {:?}", client_id, msg);
    }

    async fn handle_disconnect(&self, client_id: u64, _svr_tx: Sender<Event>) {
        debug!("Client {} disconnected", client_id);
    }

    async fn handle_client_error(&self, client_id: u64, err: Error, _svr_tx: Sender<Event>) {
        debug!("Client {} error: {}", client_id, err);
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        todo!()
    }
}
