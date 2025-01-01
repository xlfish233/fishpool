use salvo::prelude::*;
use salvo::serve_static::StaticDir;

pub fn routes() -> Router {
    Router::with_path("static").get(StaticDir::new(["static"]))
}
