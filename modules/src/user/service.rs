use super::model::{self};
use super::request::{self};
use crate::db::get_db;
use crate::response::{ApiError, ApiResponse};
use crate::result::ApiResult;
use crate::salt::{DefaultSaltGenerator, SaltGenerator};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

pub struct Service {}

impl Service {
    /// Check if service is available
    pub async fn check(&self) -> bool {
        if let Ok(_db) = get_db() {
            return true;
        }
        false
    }
    pub async fn create(&self, params: request::ReqCreate) -> ApiResult<()> {
        let db = get_db().unwrap();
        //check if user exists
        let user = model::Entity::find()
            .filter(model::Column::Username.eq(params.username.clone()))
            .one(db)
            .await?;
        if user.is_some() {
            return Err(ApiError::ParamsError("用户名已存在".to_string()));
        }
        let salt = DefaultSaltGenerator::gen_salt();
        // concat salt and password
        let salted_password = format!("{}{}", salt, params.password);
        // hash password
        let hashed_password = bcrypt::hash(salted_password, bcrypt::DEFAULT_COST).unwrap();
        let now = chrono::Utc::now().timestamp();

        let new_user = model::ActiveModel {
            username: Set(params.username),
            password: Set(hashed_password),
            salt: Set(salt),
            last_login_at: Set(now),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        //save the new user
        new_user.save(db).await?;

        Ok(ApiResponse::success())
    }
    pub async fn login(&self, params: request::ReqLogin) -> ApiResult<()> {
        let db = get_db().unwrap();
        let user = model::Entity::find()
            .filter(model::Column::Username.eq(params.username.clone()))
            .one(db)
            .await?;
        if user.is_none() {
            return Err(ApiError::ParamsError("用户名不存在".to_string()));
        }
        let user = user.unwrap();
        let salted_password = format!("{}{}", user.salt, params.password);
        if !bcrypt::verify(salted_password, &*user.password).unwrap() {
            return Err(ApiError::ParamsError("密码错误".to_string()));
        }
        //update last login time
        let now = chrono::Utc::now().timestamp();
        let update_user = model::ActiveModel {
            last_login_at: Set(now),
            ..Default::default()
        };
        update_user.update(db).await?;

        //TODO RETURN TOKEN TO USER.
        //TODO SINGLE ACCOUNT SESSION.
        Ok(ApiResponse::success())
    }
}
