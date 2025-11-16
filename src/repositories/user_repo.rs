// src/repositories/user_repo.rs
use async_trait::async_trait;
use crate::dto::user_dto::{CreateUserRequest, UpdateUserRequest};
use crate::entities::user;
use crate::errors::RepoError;

#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn create(&self, payload: CreateUserRequest) -> Result<user::Model, RepoError>;
    async fn list(&self) -> Result<Vec<user::Model>, RepoError>;
    async fn find_by_id(&self, id: i32) -> Result<user::Model, RepoError>;
    async fn update(&self, id: i32, payload: UpdateUserRequest) -> Result<user::Model, RepoError>;
    async fn delete(&self, id: i32) -> Result<(), RepoError>;
}
