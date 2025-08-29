use crate::config::db_config;
pub struct AppState {
    pub db: db_config::DatabasePool,
}