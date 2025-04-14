use crate::{domain::models::Cn649, usecases::repository::Cn649Repository};

#[derive(Clone)] 
pub struct Cn649Service<T: Cn649Repository> {
    repository: T,
}

impl<T: Cn649Repository + Send + Sync> Cn649Service<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn get_all(&self) -> Result<Vec<Cn649>, sqlx::Error> {
        self.repository.get_all().await
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Option<Cn649>, sqlx::Error> {
        self.repository.get_by_id(id).await
    }

    pub async fn create(&self, cn649: &Cn649) -> Result<(), sqlx::Error> {
        self.repository.create(cn649).await
    }

    pub async fn update(&self, id: i32, cn649: &Cn649) -> Result<(), sqlx::Error> {
        self.repository.update(id, cn649).await
    }

    pub async fn delete(&self, id: i32) -> Result<(), sqlx::Error> {
        self.repository.delete(id).await
    }
}