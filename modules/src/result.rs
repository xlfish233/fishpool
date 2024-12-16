use crate::response::{ApiError, ApiResponse};

pub type ApiResult<T> = Result<ApiResponse<T>, ApiError>;
