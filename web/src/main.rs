use anyhow::Result;

use salvo::prelude::*;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr).await?;
    let acceptor = salvo::conn::tcp::TcpAcceptor::try_from(listener)?;
    let route = Router::new().push(modules::user::router::routes());
    let service = Service::new(route);
    Server::new(acceptor).serve(service).await;

    Ok(())
}
