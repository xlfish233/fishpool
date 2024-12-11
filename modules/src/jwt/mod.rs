use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
#[derive(Serialize, Deserialize)]
pub struct JWTClaims {
    iss: String,
    exp: i64,
    sub: String,
    iat: i64,
    session_id: String,
    uid: i64,
    platform: i8,
}
static JWT_ALGORITHM: &str = "HS256";

static JWT_ISS: &str = "FISH_POOL";

static JWT_SUB: &str = "FISH_POOL";
static JWT_SECRET_KEY: &str = "FISH_POOL_DEFAULT_SECRET_KEY";
static JWT_TOKEN_LAST_TIME: i64 = 60 * 60; //1 hour

impl JWTClaims {
    pub fn new(uid: i64, platform: i8) -> Self {
        let now = chrono::Local::now();
        let exp = now.timestamp() + JWT_TOKEN_LAST_TIME;
        Self {
            iss: JWT_ISS.to_string(),
            exp,
            sub: JWT_SUB.to_string(),
            iat: now.timestamp(),
            session_id: uuid::Uuid::new_v4().to_string(),
            uid,
            platform,
        }
    }
    pub fn new_with_time(uid: i64, platform: i8, now_timestamp: i64) -> Self {
        let exp = now_timestamp + JWT_TOKEN_LAST_TIME;
        Self {
            iss: JWT_ISS.to_string(),
            exp,
            sub: JWT_SUB.to_string(),
            iat: now_timestamp,
            session_id: uuid::Uuid::new_v4().to_string(),
            uid,
            platform,
        }
    }
    pub fn get_token(&self) -> Result<String> {
        let alg = jsonwebtoken::Algorithm::from_str(JWT_ALGORITHM)?;
        let header = jsonwebtoken::Header::new(alg);
        jsonwebtoken::encode(
            &header,
            &self,
            &jsonwebtoken::EncodingKey::from_secret(JWT_SECRET_KEY.as_bytes()),
        )
        .context("Failed to encode token")
    }
    pub fn from_token(token: &str) -> Result<Self> {
        let alg = jsonwebtoken::Algorithm::from_str(JWT_ALGORITHM)?;
        let validation = jsonwebtoken::Validation::new(alg);
        let token_data = jsonwebtoken::decode::<Self>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(JWT_SECRET_KEY.as_bytes()),
            &validation,
        )
        .context("Failed to decode token")?;
        Ok(token_data.claims)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Local;

    #[test]
    fn test_new() {
        let uid = 123;
        let platform = 1;
        let claims = JWTClaims::new(uid, platform);
        let now = Local::now().timestamp();
        assert_eq!(claims.iss, "FISH_POOL");
        assert_eq!(claims.exp, now + JWT_TOKEN_LAST_TIME);
        assert_eq!(claims.sub, "FISH_POOL");
        assert!(claims.iat >= now);
        assert_eq!(claims.uid, uid);
        assert_eq!(claims.platform, platform);
        assert_ne!(claims.session_id, "");
    }

    #[test]
    fn test_get_token_and_from_token() {
        let uid = 456;
        let platform = 2;
        let claims = JWTClaims::new(uid, platform);
        let token = claims.get_token().unwrap();
        let decoded_claims = JWTClaims::from_token(&token).unwrap();
        assert_eq!(claims.iss, decoded_claims.iss);
        assert_eq!(claims.exp, decoded_claims.exp);
        assert_eq!(claims.sub, decoded_claims.sub);
        assert_eq!(claims.iat, decoded_claims.iat);
        assert_eq!(claims.session_id, decoded_claims.session_id);
        assert_eq!(claims.uid, decoded_claims.uid);
        assert_eq!(claims.platform, decoded_claims.platform);
    }

    #[test]
    fn test_from_token_with_invalid_token() {
        let invalid_token = "invalid.token";
        let result = JWTClaims::from_token(invalid_token);
        assert!(result.is_err());
    }
    #[test]
    fn test_from_token_with_expired_token() {
        let uid = 456;
        let platform = 2;
        //leeway is 60 seconds, so the token is expired after 100 seconds
        let expired_time = Local::now().timestamp() - 100 - JWT_TOKEN_LAST_TIME;
        let claims = JWTClaims::new_with_time(uid, platform, expired_time);
        let token = claims.get_token().unwrap();
        let result = JWTClaims::from_token(&token);
        assert!(result.is_err());
    }
}
