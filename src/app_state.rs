use sea_orm::DatabaseConnection;
use std::sync::Arc;
use crate::repositories::sea_user_repo::SeaUserRepository;
use crate::services::user_service::UserService;

/// Global app state kept behind Arc
pub struct AppState {
    pub user_service: Arc<UserService>,
    pub db: Arc<DatabaseConnection>,
}

impl AppState {
    pub fn new(db_conn: DatabaseConnection) -> Result<Self, anyhow::Error> {
        let db = Arc::new(db_conn);
        let user_repo = Arc::new(SeaUserRepository::new(db.clone()));
        let user_service = Arc::new(UserService::new(user_repo));
        Ok(Self {
            user_service,
            db,
        })
    }
}
