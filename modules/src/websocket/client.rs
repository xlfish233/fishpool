use anyhow::Result;
use futures_util::StreamExt;
use salvo::websocket::Message;
use salvo::websocket::WebSocket;

use tokio::sync::oneshot;

pub struct WsClient {
    inner: WebSocket,
    id: Option<u64>,
    //channel to tell owner this client is disconnected
    disconnect_ch: Option<oneshot::Sender<()>>,
    //channel to stop listen task.
    stop_ch: Option<oneshot::Sender<()>>,
}

impl WsClient {
    pub fn new(inner: WebSocket) -> Self {
        Self {
            inner,
            id: None,
            disconnect_ch: None,
            stop_ch: None,
        }
    }
    pub async fn start_listen(&mut self) {
        // First message is always message from client using token/session to identify client.
        if let Some(msg) = self.inner.next().await {
            match msg {
                Ok(msg) => {
                    match msg {
                       text => {
                           let id = text.to_str().unwrap().parse::<u64>().unwrap();
                           self.id = Some(id);
                           self.disconnect_ch = Some(oneshot::channel().0);
                           self.stop_ch = Some(oneshot::channel().0);
                       }
                    }
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }
        }
        if self.id.is_none() {
            self.disconnect_ch.take().unwrap().send(()).unwrap();
        }
        return;
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
