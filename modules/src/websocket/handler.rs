use super::server::{Event, WS_SERVICE_SENDER};

use salvo::http::StatusError;
use salvo::prelude::*;
use serde_json::json;
#[handler]
pub async fn connect(req: &mut Request, res: &mut Response) -> Result<(), StatusError> {
    let ws_upgrade = WebSocketUpgrade::new(); // Bind the temporary value to a variable
    let ws = ws_upgrade
        .upgrade(req, res, |ws| async move {
            WS_SERVICE_SENDER.send(Event::Upgrade(ws)).await.ok();
        })
        .await;
    if let Err(e) = ws {
        tracing::error!("websocket upgrade error: {:?}", e);
        res.render(Json(json!({
            "code": 500,
            "msg": "websocket upgrade error",
            "data": e.to_string()
        })));
        return Ok(());
    }

    Ok(())
}

