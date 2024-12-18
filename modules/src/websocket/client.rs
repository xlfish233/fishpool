use anyhow::Result;
use salvo::websocket::Message;
use salvo::websocket::WebSocket;

pub struct WsClient {
    inner: WebSocket,
    id: Option<u64>,
    //channel to tell owner this client is disconnected
}

impl WsClient {
    pub fn new(inner: WebSocket) -> Self {
        Self { inner, id: None }
    }
    pub fn verified(&self) -> bool {
        self.id.is_some()
    }
    pub async fn send(&mut self, msg: Message) -> Result<()> {
        self.inner.send(msg).await?;
        Ok(())
    }
}
