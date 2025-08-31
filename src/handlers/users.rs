use actix_web::{ error, web, Responder };
use serde_json::json;
use crate::{app_state::AppState, models::users::{self, UsersDTO}, utils::api_response::ApiResponse};

pub fn config (cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users")
            .route(web::get().to(get_all))
            .route(web::post().to(create))
    );
}

async fn get_all(pool: web::Data<AppState>) -> impl Responder {
    let mut conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(err) => return ApiResponse::error(&err.to_string()),
    };

    match users::Users::get_all(&mut conn) {
        Ok(content) => ApiResponse::success(json!(content)),
        Err(err) => ApiResponse::error(&err.to_string())
    }
}

async fn create(pool: web::Data<AppState>, user: web::Json<UsersDTO>) -> impl Responder {
    let mut conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(err) => return ApiResponse::error(&err.to_string()),
    };

    match users::Users::create(&mut conn, &user) {
        Ok(content) => ApiResponse::success(json!(content)),
        Err(err) => ApiResponse::error(&err.to_string())
    }
}