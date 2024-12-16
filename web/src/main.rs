use modules::db;

#[tokio::main]
async fn main() -> Result<()> {
    db::init().await?;
    Ok(())
}
