use crate::result::ApiResult;
use salvo::prelude::*;
use serde::{Deserialize, Serialize};

use salvo::Writer;

#[derive(Serialize, Deserialize)]
pub enum ApiError {
    ParamsError(String),
}

impl From<salvo::http::ParseError> for ApiError {
    // 添加此实现
    fn from(err: salvo::http::ParseError) -> Self {
        ApiError::ParamsError(err.to_string())
    }
}

impl Into<ApiError> for validator::ValidationErrors {
    fn into(self) -> ApiError {
        ApiError::ParamsError(self.to_string())
    }
}

impl<M> Into<ApiResult<M>> for salvo::http::ParseError
where
    M: Serialize + Send + Writer,
{
    fn into(self) -> ApiResult<M> {
        ApiResult::Err(ApiError::ParamsError(self.to_string()))
    }
}

impl<M> Into<ApiResult<M>> for validator::ValidationErrors
where
    M: Serialize + Send + Writer,
{
    fn into(self) -> ApiResult<M> {
        ApiResult::Err(ApiError::ParamsError(self.to_string()))
    }
}

#[async_trait]
impl Writer for ApiError {
    async fn write(self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        match self {
            _ => {
                res.render(Json(self));
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
        ApiResult::Ok(res)
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
}

impl<T> Into<ApiResponse<T>> for ApiError
where
    T: Serialize + Send,
{
    fn into(self) -> ApiResponse<T> {
        ApiResponse {
         code: 0,
            msg: serde_json::to_string(&self).unwrap(),
            data: None,
        }
    }
}
