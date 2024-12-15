use crate::db::get_db;
use crate::user::service::{Service, ServiceTrait};
use anyhow::{Context, Result};
use async_trait::async_trait;
use sea_orm::prelude::async_trait;

#[async_trait]
impl ServiceTrait for Service {
    fn new() -> Self {
        Self {}
    }

    fn check() -> Result<()> {
        let _db = get_db().context("db not init")?;
        //todo! check table is right.
        let service = Self::new();
        Ok(())
    }
}
#[async_trait]
pub trait LocalImplExt {}

#[async_trait]
impl LocalImplExt for Service {}
