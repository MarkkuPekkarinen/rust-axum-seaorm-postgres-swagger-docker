use std::sync::Arc;
use crate::repositories::UserRepository;
use crate::dto::user_dto::{CreateUserRequest, UpdateUserRequest, UserResponse};
use crate::errors::ServiceError;

pub struct UserService {
    repo: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(repo: Arc<dyn UserRepository>) -> Self {
        Self { repo }
    }

    pub async fn create_user(&self, req: CreateUserRequest) -> Result<UserResponse, ServiceError> {
        let model = self.repo.create(req).await.map_err(ServiceError::from)?;
        Ok(model.into())
    }

    pub async fn list_users(&self) -> Result<Vec<UserResponse>, ServiceError> {
        let users = self.repo.list().await.map_err(ServiceError::from)?;
        Ok(users.into_iter().map(|u| u.into()).collect())
    }

    pub async fn get_user(&self, id: i32) -> Result<UserResponse, ServiceError> {
        let u = self.repo.find_by_id(id).await.map_err(ServiceError::from)?;
        Ok(u.into())
    }

    pub async fn update_user(&self, id: i32, payload: UpdateUserRequest) -> Result<UserResponse, ServiceError> {
        let u = self.repo.update(id, payload).await.map_err(ServiceError::from)?;
        Ok(u.into())
    }

    pub async fn delete_user(&self, id: i32) -> Result<(), ServiceError> {
        self.repo.delete(id).await.map_err(ServiceError::from)
    }
}
