use crate::domain::models::Cn649;
use async_trait::async_trait;

#[async_trait]
pub trait Cn649Repository: Send + Sync + 'static{
    async fn get_all(&self) -> Result<Vec<Cn649>, sqlx::Error>;
    async fn get_by_id(&self, id: i32) -> Result<Option<Cn649>, sqlx::Error>;
    async fn create(&self, cn649: &Cn649) -> Result<(), sqlx::Error>;
    async fn update(&self, id: i32, cn649: &Cn649) -> Result<(), sqlx::Error>;
    async fn delete(&self, id: i32) -> Result<(), sqlx::Error>;
}