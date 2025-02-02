use anyhow::Result;

use salvo::prelude::*;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr).await?;
    let acceptor = salvo::conn::tcp::TcpAcceptor::try_from(listener)?;
    let mut route = Router::new();
    route = route.push(modules::user::router::routes());
    route = route.push(modules::websocket::router::routes());
    route = route.push(modules::static_files::router::routes());
    let service = Service::new(route);
    Server::new(acceptor).serve(service).await;

    Ok(())
}
