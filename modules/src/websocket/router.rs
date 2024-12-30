use super::handler::connect;
use salvo::prelude::*;
pub fn routes() -> Router {
    Router::with_path("ws").push(Router::with_path("upgrade").get(connect))
}
