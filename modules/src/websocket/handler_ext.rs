use super::server::WsEventHandler;
use anyhow::Error;
use salvo::prelude::*;
use salvo::websocket::{Message, WebSocket};

pub struct HandlerExt {}

#[async_trait]
impl WsEventHandler for HandlerExt {
    async fn handle_upgraded_ws(&self, ws: WebSocket) {
        todo!()
    }

    async fn handle_message(&self, client_id: u64, msg: Message) {
        todo!()
    }

    async fn handle_disconnect(&self, client_id: u64) {
        todo!()
    }

    async fn handle_client_error(&self, client_id: u64, err: Error) {
        todo!()
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        todo!()
    }
}
