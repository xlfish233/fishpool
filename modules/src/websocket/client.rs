use super::Event;
use anyhow::Result;
use futures_util::SinkExt;
use salvo::websocket::{Message, WebSocket};
use tokio::sync::mpsc::Sender;
#[derive(Debug)]
pub struct WsClient {
    sock: WebSocket,
    cid: u64,
    user_id: Option<u64>,
    svr_event_tx: Sender<Event>,
}

impl WsClient {
    pub fn new(inner: WebSocket, svr_event_tx: Sender<Event>, cid: u64) -> Self {
        Self {
            cid,
            sock: inner,
            user_id: None,
            svr_event_tx,
        }
    }

    // pub async fn start_listen(self, mut msg_rx: Receiver<Message>) {
    //
    //
    //     let send_task = tokio::spawn(async move {
    //         while let Some(msg) = msg_rx.recv().await {
    //             if let Err(e) = ws_write.send(msg).await {
    //                 error!("发送消息错误: {:?}", e);
    //                 break;
    //             }
    //         }
    //     });
    //
    //     let receive_task = tokio::spawn(async move {
    //         while let Some(msg) = ws_read.next().await {
    //             match msg {
    //                 Ok(msg) => {
    //                     if msg.is_close() {
    //                         break;
    //                     }
    //                     // TODO: 处理消息
    //                 }
    //                 Err(e) => {
    //                     error!("接收消息错误: {:?}", e);
    //                     break;
    //                 }
    //             }
    //         }
    //     });
    //
    //     tokio::select! {
    //         _ = send_task => {},
    //         _ = receive_task => {},
    //     }
    //
    //     if let Some(tx) = &self.disconnect_ch {
    //         let _ = tx.send(self.cid).await;
    //     }
    // }

    async fn handle_message(&self, _msg: Message) -> Result<()> {
        // TODO: 实现消息处理逻辑
        Ok(())
    }

    pub fn verified(&self) -> bool {
        self.user_id.is_some()
    }

    pub async fn close(&mut self) {
        self.svr_event_tx
            .send(Event::Disconnect(self.cid))
            .await
            .map(|_| tracing::info!("client disconnect"))
            .ok();
    }

    pub fn cid(&self) -> u64 {
        self.cid
    }
}
