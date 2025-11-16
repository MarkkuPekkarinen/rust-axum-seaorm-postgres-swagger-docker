use thiserror::Error;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Error, Debug)]
pub enum RepoError {
    #[error("not found")]
    NotFound,
    #[error("duplicate")]
    Duplicate,
    #[error("db error")]
    Db(#[from] sea_orm::DbErr),
}

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("not found")]
    NotFound,
    #[error("conflict")]
    Conflict,
    #[error("internal")]
    Internal,
}

impl From<RepoError> for ServiceError {
    fn from(e: RepoError) -> Self {
        match e {
            RepoError::NotFound => ServiceError::NotFound,
            RepoError::Duplicate => ServiceError::Conflict,
            RepoError::Db(_) => ServiceError::Internal,
        }
    }
}

impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        let (code, message) = match &self {
            ServiceError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            ServiceError::Conflict => (StatusCode::CONFLICT, self.to_string()),
            ServiceError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };
        let body = json!({ "error": message });
        (code, axum::Json(body)).into_response()
    }
}
