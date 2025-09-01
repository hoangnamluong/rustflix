use actix_web::{ web, Responder };
use serde_json::json;
use crate::{
    app_state::AppState,
    models::title::TitleCreateDTO,
    utils::api_response::ApiResponse
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/titles")
            .route(web::get().to(get_all))
            .route(web::post().to(create))
    );
}

async fn get_all(pool: web::Data<AppState>) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    match crate::services::title::get_all(conn).await {
        Ok(titles) => ApiResponse::success(json!(titles)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}

async fn create(pool: web::Data<AppState>, title: web::Json<TitleCreateDTO>) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    match crate::services::title::create(conn, title.into_inner()).await {
        Ok(titles) => ApiResponse::success(json!(titles)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}
