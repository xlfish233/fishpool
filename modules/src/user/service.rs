use anyhow::Result;
use sea_orm::prelude::async_trait;

pub struct Service {}

// Write as trait is for future extension.
#[async_trait::async_trait]
pub trait ServiceTrait {
    fn new() -> Self;
    fn check()-> Result<()>;
}
