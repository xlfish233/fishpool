use super::server::Event;

use salvo::websocket::{Message, WebSocket};
use tokio::select;
use tokio::sync::mpsc::{Receiver, Sender};

#[derive(Debug)]
pub struct WsClient {
    sock: WebSocket,
    cid: u64,
    user_id: Option<u64>,
    svr_event_tx: Sender<Event>,
    cmd_rx: Receiver<Command>,
}
pub enum Command {
    Send(Message),
    Close,
}

impl WsClient {
    pub fn new(
        inner: WebSocket,
        svr_event_tx: Sender<Event>,
        cid: u64,
        cmd_rx: Receiver<Command>,
    ) -> Self {
        Self {
            cid,
            sock: inner,
            user_id: None,
            svr_event_tx,
            cmd_rx,
        }
    }
    pub async fn serve(&mut self) {
        loop {
            select! {
                msg = self.sock.recv() => {
                    if let Some(msg) = msg {
                        if let Ok(msg) = msg {
                            self.svr_event_tx
                                .send(Event::Message(self.cid, msg))
                                .await
                                .map(|_| tracing::info!("client disconnect"))
                                .ok();
                        }
                    }
                    break;

                },
                cmd = self.cmd_rx.recv() => {
                    match cmd {
                        Some(Command::Send(msg)) => {
                            if let Err(e) = self.sock.send(msg).await {
                                self.svr_event_tx
                                    .send(Event::ClientError(self.cid, e.into()))
                                    .await
                                    .map(|_| tracing::info!("client disconnect"))
                                    .ok();
                                break;
                            }
                        }
                        Some(Command::Close) => {
                            // fuck
                            break;
                        }
                        None => {
                            break;
                        }
                    };
                }
            }
        }
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
