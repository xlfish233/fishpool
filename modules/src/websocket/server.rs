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
    async fn send_text_by_id(&self, uid: u64, msg: &str) -> Result<()> {
        let mut cli = self.clients.get_mut(&uid).unwrap();
        cli.send(Message::text(msg)).await?;
        Ok(())
    }
    async fn broadcast_text(&self, msg: &str) -> Result<()> {
        for mut cli in self.clients.iter_mut() {
            cli.send(Message::text(msg)).await?;
        }
        Ok(())
    }
    async fn kick_by_id(&self, uid: u64) -> Result<()> {
        let mut cli = self.clients.get_mut(&uid).unwrap();
        cli.close();
        self.clients.remove(&uid);
        Ok(())
    }
}
