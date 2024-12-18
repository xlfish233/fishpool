use super::model::{self};
use super::request::{self};
use crate::crypto::{DefaultSaltGenerator, SaltGenerator};
use crate::db::get_db;
use crate::jwt::JWTClaims;
use crate::response::{ApiError, ApiResponse};
use crate::result::ApiResult;
use crate::user::response::RespLogin;
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
    pub async fn create(params: request::ReqCreate) -> ApiResult<()> {
        let db = get_db()?;
        let user = model::Entity::find()
            .filter(model::Column::Username.eq(params.username.clone()))
            .one(db)
            .await?;
        if user.is_some() {
            return Err(ApiError::ParamsError("用户名已存在".to_string()));
        }
        let (salt, hash) = DefaultSaltGenerator::gen_salt_pair(&params.password);
        let now = chrono::Utc::now().timestamp();

        let new_user = model::ActiveModel {
            username: Set(params.username),
            password_hash: Set(hash),
            salt: Set(salt),
            last_login_at: Set(now),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        new_user.save(db).await?;

        Ok(ApiResponse::success())
    }
    pub async fn login(params: request::ReqLogin) -> ApiResult<RespLogin> {
        let db = get_db()?;
        let user = model::Entity::find()
            .filter(model::Column::Username.eq(params.username.clone()))
            .one(db)
            .await?;
        if user.is_none() {
            return Err(ApiError::ParamsError("用户名不存在".to_string()));
        }
        let user = user.unwrap();
        if !DefaultSaltGenerator::verify(&user.salt, &params.password, &user.password_hash) {
            return Err(ApiError::ParamsError("密码错误".to_string()));
        }
        //update last login time
        let now = chrono::Utc::now().timestamp();
        let update_user = model::ActiveModel {
            last_login_at: Set(now),
            ..Default::default()
        };
        update_user.update(db).await?;
        let token = JWTClaims::new(user.id, 1);
        let token = token.get_token()?;
        Ok(ApiResponse::new(
            0,
            "登录成功".to_string(),
            Some(RespLogin {
                token,
                session_id: uuid::Uuid::new_v4().to_string(),
            }),
        ))
    }
}
