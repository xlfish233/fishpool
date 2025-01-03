use crate::result::ApiResult;

use salvo::Writer;
use salvo::prelude::*;
use sea_orm::DbErr;
use serde::{Deserialize, Serialize};
use tracing::error;

#[derive(Serialize, Deserialize)]
pub enum ApiError {
    ParamsError(String),
    ServerError(String),
}

impl From<salvo::http::ParseError> for ApiError {
    // 添加此实现
    fn from(err: salvo::http::ParseError) -> Self {
        ApiError::ParamsError(err.to_string())
    }
}

impl From<validator::ValidationErrors> for ApiError {
    fn from(val: validator::ValidationErrors) -> Self {
        ApiError::ParamsError(val.to_string())
    }
}

#[async_trait]
impl Writer for ApiError {
    async fn write(self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        match self {
            ApiError::ParamsError(msg) => {
                error!("ApiError::ParamsError: {}", msg);

                res.render(Json(ApiResponse::<()>::msg(10000, msg)));
            }
            ApiError::ServerError(msg) => {
                error!("ApiError::ServerError: {}", msg);

                res.render(Json(ApiResponse::<()>::msg(
                    50000,
                    "Service internal error".into(),
                )));
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<D>
where
    D: Serialize + Send,
{
    pub code: i32,
    pub msg: String,
    pub data: Option<D>,
}

impl<D> From<ApiResponse<D>> for ApiResult<D>
where
    D: Serialize + Writer + Send,
{
    fn from(res: ApiResponse<D>) -> Self {
        Ok(res)
    }
}

impl<D> ApiResponse<D>
where
    D: Serialize + Send,
{
    pub fn new(code: i32, msg: String, data: Option<D>) -> Self {
        ApiResponse { code, msg, data }
    }
    pub fn msg(code: i32, msg: String) -> Self {
        ApiResponse {
            code,
            msg,
            data: None,
        }
    }
    pub fn success() -> Self {
        ApiResponse {
            code: 0,
            msg: "success".to_string(),
            data: None,
        }
    }
}

impl<T> From<ApiError> for ApiResponse<T>
where
    T: Serialize + Send,
{
    fn from(val: ApiError) -> Self {
        ApiResponse {
            code: 0,
            msg: serde_json::to_string(&val).unwrap(),
            data: None,
        }
    }
}

#[async_trait]
impl<T> Writer for ApiResponse<T>
where
    T: Serialize + Send,
{
    async fn write(self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        res.render(Json(self));
    }
}

impl From<DbErr> for ApiError {
    fn from(e: DbErr) -> Self {
        error!("{:?}", e);
        ApiError::ServerError(e.to_string())
    }
}
impl From<anyhow::Error> for ApiError {
    fn from(e: anyhow::Error) -> Self {
        error!("{:?}", e);
        ApiError::ServerError(e.to_string())
    }
}
