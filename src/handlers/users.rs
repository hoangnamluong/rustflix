use actix_web::{ error, web, Responder };
use serde_json::json;
use crate::{app_state::AppState, config::db_config::DatabaseConn, models::users::{self, UsersCreateDTO, UsersUpdateDTO}, utils::api_response::ApiResponse};

pub fn config (cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users")
            .route(web::get().to(get_all))
            .route(web::post().to(create))
    );
}

async fn get_all(pool: web::Data<AppState>) -> impl Responder {
    // Get database connection from pool
    let mut conn: DatabaseConn = match pool.db.get() {
        Ok(conn) => conn,
        Err(err) => return ApiResponse::error("Failed to connect to database"),
    };

    // Execute database query in a separate thread
    let result = web::block(move || users::Users::get_all(&mut conn))
        .await
        .map_err(|err| format!("Error executing database query: {}", err))
        .and_then(|r| r.map_err(|e| e.to_string()));

    // Return API response based on result
    match result {
        Ok(users) => ApiResponse::success(json!(users)),
        Err(error_message) => ApiResponse::error(&error_message),
    }
}

async fn create(pool: web::Data<AppState>, user: web::Json<UsersCreateDTO>) -> impl Responder {
    let mut conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(err) => return ApiResponse::error(&err.to_string()),
    };

    let result = web::block(move || {
        users::Users::create(&mut conn, &user)
    })
    .await
    .map_err(|err| format!("Error executing database query: {}", err))
    .and_then(|r| r.map_err(|e| e.to_string()));

    match result {
        Ok(content) => ApiResponse::success(json!(content)),
        Err(err) => ApiResponse::error(&err.to_string())
    }
}

async fn update(pool: web::Data<AppState>, user: web::Json<UsersUpdateDTO>) {
    
}