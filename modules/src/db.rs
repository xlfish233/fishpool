use anyhow::{Context, Result};
use sea_orm::DatabaseConnection;
use std::sync::OnceLock;

static DB: OnceLock<DatabaseConnection> = OnceLock::new();

pub async fn init() {}

pub fn get_db<'a>() -> Result<&'a DatabaseConnection> {
    //here do not init,if not init,return error
    DB.get().context("db not init")
}
