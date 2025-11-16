// src/dto/user_dto.rs
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::entities::user;

#[derive(Deserialize, ToSchema)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct UserResponse {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<user::Model> for UserResponse {
    fn from(m: user::Model) -> Self {
        Self {
            id: m.id,
            name: m.name,
            email: m.email,
            created_at: m.created_at.to_string(),
            updated_at: m.updated_at.to_string(),
        }
    }
}
