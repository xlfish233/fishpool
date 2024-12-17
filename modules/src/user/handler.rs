use crate::response::ApiResponse;
use crate::result::ApiResult;
use crate::user::request::ReqLogin;
use crate::user::response::RespLogin;
use crate::user::service;
use salvo::prelude::*;
// #[handler]
// pub async fn create(req: &mut Request) -> ApiResult<()> {
//     let params = req.parse_queries::<ReqCreate>()?;
//     Ok(ApiResponse::msg(0, "create".parse().unwrap()))
// }

#[handler]
pub async fn asdf() -> impl Writer {
    ApiResponse::<()>::msg(0, "create".parse().unwrap())
}


#[handler]
pub async fn login(req: &mut Request) -> ApiResult<RespLogin> {
    let params = req.parse_json::<ReqLogin>().await?;
    service::Service::login(params).await
}

