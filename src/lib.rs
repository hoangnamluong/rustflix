pub mod utils;
pub mod schema;
pub mod config;
pub mod handlers;
pub mod models;
pub mod repositories;
pub mod services;
pub mod app_state;

pub type RepoResult<F> = Result<F, actix_web::Error>;