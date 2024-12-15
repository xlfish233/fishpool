mod jwt;

mod user;

mod response;
mod result;

mod db;

use salvo::prelude::*;
use salvo::session::{Session, SessionDepotExt};
#[handler]
pub async fn login(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    if req.method() == salvo::http::Method::POST {
        let mut session = Session::new();
        session
            .insert("username", req.form::<String>("username").await.unwrap())
            .unwrap();
        depot.set_session(session);

        res.render(Redirect::other("/"));
    } else {
        res.render(Text::Html("username"));
    }
}
#[handler]
pub async fn logout(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    depot.remove_session();
    res.render(Redirect::other("/"));
}