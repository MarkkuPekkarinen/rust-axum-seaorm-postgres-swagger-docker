// src/routes/users.rs
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use std::sync::Arc;

use crate::dto::user_dto::{CreateUserRequest, UpdateUserRequest, UserResponse};
use crate::errors::ServiceError;
use crate::app_state::AppState;

/// AppState reference type used by handlers
type AppStateRef = Arc<AppState>;

/// Create user
#[utoipa::path(
    post,
    path = "/users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created", body = UserResponse),
        (status = 409, description = "Conflict"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "Users"
)]
pub async fn create_user(
    State(state): State<AppStateRef>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>), ServiceError> {
    let svc = &state.user_service;
    let res = svc.create_user(payload).await?;
    Ok((StatusCode::CREATED, Json(res)))
}

/// List users
#[utoipa::path(
    get,
    path = "/users",
    responses(
        (status = 200, description = "List users", body = [UserResponse]),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "Users"
)]
pub async fn list_users(
    State(state): State<AppStateRef>,
) -> Result<Json<Vec<UserResponse>>, ServiceError> {
    let svc = &state.user_service;
    let res = svc.list_users().await?;
    Ok(Json(res))
}

/// Get user
#[utoipa::path(
    get,
    path = "/users/{id}",
    params(
        ("id" = i32, Path, description = "User id")
    ),
    responses(
        (status = 200, description = "User detail", body = UserResponse),
        (status = 404, description = "Not found"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "Users"
)]
pub async fn get_user(
    State(state): State<AppStateRef>,
    Path(id): Path<i32>,
) -> Result<Json<UserResponse>, ServiceError> {
    let svc = &state.user_service;
    let res = svc.get_user(id).await?;
    Ok(Json(res))
}

/// Update user
#[utoipa::path(
    put,
    path = "/users/{id}",
    params(
        ("id" = i32, Path, description = "User id")
    ),
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "User updated", body = UserResponse),
        (status = 404, description = "Not found"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "Users"
)]
pub async fn update_user(
    State(state): State<AppStateRef>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, ServiceError> {
    let svc = &state.user_service;
    let res = svc.update_user(id, payload).await?;
    Ok(Json(res))
}

/// Delete user
#[utoipa::path(
    delete,
    path = "/users/{id}",
    params(
        ("id" = i32, Path, description = "User id")
    ),
    responses(
        (status = 204, description = "User deleted"),
        (status = 404, description = "Not found"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "Users"
)]
pub async fn delete_user(
    State(state): State<AppStateRef>,
    Path(id): Path<i32>,
) -> Result<StatusCode, ServiceError> {
    let svc = &state.user_service;
    svc.delete_user(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
