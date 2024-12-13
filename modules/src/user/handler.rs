use crate::result::ApiResult;
use crate::user::req::ReqCreate;
use salvo::prelude::*;

#[handler]
pub async fn create(req: &mut Request) -> ApiResult<()> {
    match req.parse_queries::<ReqCreate>() {
        Ok(params) => {
            tracing::info!("params: {:?}", params);
            ApiResult::msg("create")
        }
        Err(e) => e.into(),
    }
}
