use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RespLogin {
    pub token: String,
    pub session_id: String,
}
