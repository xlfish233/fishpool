use crate::db::get_db;
use crate::response::{ApiError, ApiResponse};
use crate::result::ApiResult;
use crate::salt::{DefaultSaltGenerator, SaltGenerator};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};

pub struct Service {}

impl Service {
    /// Check if service is available
    pub async fn check(&self) -> bool {
        if let Ok(_db) = get_db() {
            return true;
        }
        false
    }
    pub async fn create(&self, params: crate::user::req::ReqCreate) -> ApiResult<()> {
        let db = get_db().unwrap();
        //check if user exists
        let user = crate::user::model::Entity::find()
            .filter(crate::user::model::Column::Username.eq(params.username.clone()))
            .one(db)
            .await
            .map_err(|e| ApiError::ServerError(e.to_string()))?;
        if user.is_some() {
            return Err(ApiError::ParamsError("用户名已存在".to_string()));
        }
        let salt = DefaultSaltGenerator::gen_salt();
        // concat salt and password
        let salted_password = format!("{}{}", salt, params.password);
        // hash password
        let hashed_password = bcrypt::hash(salted_password, bcrypt::DEFAULT_COST).unwrap();
        let now = chrono::Utc::now().timestamp();

        let new_model = crate::user::model::ActiveModel {
            username: Set(params.username),
            password: Set(hashed_password),
            salt: Set(salt),
            last_login_at: Set(now),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };

        Ok(ApiResponse::success())
    }
}
