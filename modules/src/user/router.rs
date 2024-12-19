use super::handler::*;
use salvo::prelude::*;
pub fn routes() -> Router {
    Router::with_path("user")
        .push(Router::with_path("/login").post(login))
        .push(Router::with_path("/create").post(create))
}
