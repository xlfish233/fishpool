use salvo::http::StatusError;
use salvo::prelude::*;

#[handler]
async fn connect(req: &mut Request, res: &mut Response) -> Result<(), StatusError> {
    WebSocketUpgrade::new()
        .upgrade(req, res, |mut ws| async move {
            while let Some(msg) = ws.recv().await {
                let msg = if let Ok(msg) = msg {
                    msg
                } else {
                    // client disconnected
                    return;
                };

                if ws.send(msg).await.is_err() {
                    // client disconnected
                    return;
                }
            }
        })
        .await
}
