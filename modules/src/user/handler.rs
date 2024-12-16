use crate::response::ApiResponse;
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
