use super::server::Event;

use crate::jwt::JWTClaims;
use anyhow::{Context, Result};
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
    state: AuthenticateState,
}
pub enum Command {
    Send(Message),
    Close,
}
#[derive(Debug)]
pub enum AuthenticateState {
    Wait,
    Authenticated,
    Failed,
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
            state: AuthenticateState::Wait,
        }
    }
    pub async fn check_authed(&mut self, msg: &Message) -> bool {
        match self.state {
            AuthenticateState::Wait => self.do_auth(msg).is_ok(),
            AuthenticateState::Authenticated => true,
            AuthenticateState::Failed => false,
        }
    }

    fn do_auth(&mut self, msg: &Message) -> Result<()> {
        let token = msg.to_str().context("Failed to decode message")?;

        let claims = JWTClaims::from_token(token)?;
        self.user_id = Some(claims.uid);
        self.state = AuthenticateState::Authenticated;
        Ok(())
    }

    pub async fn serve(&mut self) {
        loop {
            select! {
                msg = self.sock.recv() => {
                    if let Some(Ok(ws_msg)) = msg {
                        self.svr_event_tx
                                .send(Event::Message(self.cid, ws_msg))
                                .await
                                .map(|_| tracing::info!("client disconnect"))
                                .ok();
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
                            break;
                        }
                        None => {
                            // Sender is dropped
                            break;
                        }
                    };
                }
            }
        }
        //Tell the server that this client has disconnected
        self.svr_event_tx
            .send(Event::Disconnect(self.cid))
            .await
            .map(|_| tracing::info!("client disconnect"))
            .ok();
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
