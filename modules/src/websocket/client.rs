use anyhow::Result;
use futures_util::StreamExt;
use salvo::websocket::Message;
use salvo::websocket::WebSocket;

use tokio::sync::{mpsc::Sender, oneshot};

pub struct WsClient {
    inner: WebSocket,
    cid: u64,
    id: Option<u64>,
    //channel to tell owner this client is disconnected
    disconnect_ch: Option<Sender<u64>>,
    //channel to stop listen task.
    stop_ch: Option<oneshot::Sender<()>>,
}

impl WsClient {
    pub fn new(inner: WebSocket, disconnect_tx: Sender<u64>, cid: u64) -> Self {
        Self {
            cid,
            inner,
            id: None,
            disconnect_ch: Some(disconnect_tx),
            stop_ch: None,
        }
    }
    pub async fn start_listen(&mut self) {
        // First message is always message from client using token/session to identify client.
        if let Some(msg) = self.inner.next().await {
            match msg {
                Ok(msg) => {
                    let id = msg.to_str().unwrap().parse::<u64>().unwrap();
                    self.id = Some(id);
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }
        }
        if self.id.is_none() {
            //send id to disconnect channel
            if let Some(tx) = self.disconnect_ch.take() {
                tx.send(self.cid).await.unwrap();
            }
            return;
        }
    }
    pub fn verified(&self) -> bool {
        self.id.is_some()
    }
    pub async fn send(&mut self, msg: Message) -> Result<()> {
        self.inner.send(msg).await?;
        Ok(())
    }
    pub async fn close(&mut self) {
        //
    }
}
