use crate::response::{ApiError, ApiResponse};
use salvo::prelude::*;
use serde::Serialize;

pub enum ApiResult<T>
where
    T: Writer + Serialize + Send,
{
    Ok(ApiResponse<T>),
    Err(ApiError),
}

impl<T> ApiResult<T>
where
    T: Writer + Serialize + Send,
{
    pub fn msg(msg: &str) -> Self {
        ApiResult::Ok(ApiResponse::msg(0, msg.to_string()))
    }
    pub fn ok(data: T) -> Self {
        ApiResult::Ok(ApiResponse::new(0, "ok".to_string(), Some(data)))
    }
    pub fn err(data: ApiError) -> Self {
        ApiResult::Err(data)
    }
    pub fn is_ok(&self) -> bool {
        matches!(self, ApiResult::Ok(_))
    }
    pub fn is_err(&self) -> bool {
        matches!(self, ApiResult::Err(_))
    }
}

#[async_trait]
impl<D> Writer for ApiResult<D>
where
    D: Serialize + Writer + Send,
{
    async fn write(self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        match self {
            ApiResult::Ok(data) => {
                res.render(Json(data));
            }
            ApiResult::Err(data) => {
                res.render(Json(data));
            }
        }
    }
}
