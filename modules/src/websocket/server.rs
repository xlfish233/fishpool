use super::client::WsClient;
use anyhow::Result;
use dashmap::DashMap;
use salvo::websocket::Message;
struct WsServer {
    clients: DashMap<u64, WsClient>,
    //TODO BROADCAST. SEND TO ID
}

impl WsServer {
    pub fn new() -> Self {
        Self {
            clients: DashMap::new(),
        }
    }
    async fn send_by_id(&self, uid: u64) -> Result<()> {
        let mut cli = self.clients.get_mut(&uid).unwrap();
        cli.send(Message::text("hello")).await?;
        Ok(())
    }
}
