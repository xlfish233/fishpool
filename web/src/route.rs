use salvo::prelude::*;

pub fn routes() -> Router {
    Router::new().push(Router::with_path("/api/auth/login").post(modules::user::handler::login))
}
