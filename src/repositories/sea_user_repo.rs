// src/repositories/sea_user_repo.rs
use super::user_repo::UserRepository;
use crate::dto::user_dto::{CreateUserRequest, UpdateUserRequest};
use crate::entities::user;
use sea_orm::{DatabaseConnection, ActiveModelTrait, Set, EntityTrait};
use async_trait::async_trait;
use crate::errors::RepoError;
use std::sync::Arc;


pub struct SeaUserRepository {
    db: Arc<DatabaseConnection>,
}

impl SeaUserRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self { Self { db } }
}

#[async_trait]
impl UserRepository for SeaUserRepository {
    async fn create(&self, payload: CreateUserRequest) -> Result<user::Model, RepoError> {
        let now = chrono::Utc::now().naive_utc();
        let active = user::ActiveModel {
            name: Set(payload.name),
            email: Set(payload.email),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };

        active.insert(&*self.db).await.map_err(|e| {
            let s = e.to_string();
            if s.contains("duplicate key") || s.contains("unique constraint") {
                RepoError::Duplicate
            } else {
                RepoError::Db(e)
            }
        })
    }

    async fn list(&self) -> Result<Vec<user::Model>, RepoError> {
        user::Entity::find().all(&*self.db).await.map_err(RepoError::Db)
    }

    async fn find_by_id(&self, id: i32) -> Result<user::Model, RepoError> {
        user::Entity::find_by_id(id)
            .one(&*self.db)
            .await
            .map_err(RepoError::Db)?
            .ok_or(RepoError::NotFound)
    }

    async fn update(&self, id: i32, payload: UpdateUserRequest) -> Result<user::Model, RepoError> {
        let model = self.find_by_id(id).await?;
        let mut active: user::ActiveModel = model.into();

        if let Some(name) = payload.name {
            active.name = Set(name);
        }
        if let Some(email) = payload.email {
            active.email = Set(email);
        }
        active.updated_at = Set(chrono::Utc::now().naive_utc());

        active.update(&*self.db).await.map_err(RepoError::Db)
    }

    async fn delete(&self, id: i32) -> Result<(), RepoError> {
        let model = self.find_by_id(id).await?;
        let active: user::ActiveModel = model.into();
        active.delete(&*self.db).await.map_err(RepoError::Db)?;
        Ok(())
    }
}
