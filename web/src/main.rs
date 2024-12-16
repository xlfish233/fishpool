use anyhow::Result;
use modules::db;
#[tokio::main]
async fn main() -> Result<()> {
    //TODO GET CONFIG.
    db::init().await;
    Ok(())
}
