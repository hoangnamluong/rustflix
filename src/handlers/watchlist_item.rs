use actix_web::{ web, Responder };
use serde_json::json;
use crate::{
    app_state::AppState,
    config::db_config::DatabaseConn,
    models::watchlist_item::{self, WatchlistItemCreateDTO},
    utils::api_response::ApiResponse
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/watchlist-items")
            .route(web::get().to(get_all))
            .route(web::post().to(create))
    );
}

async fn get_all(pool: web::Data<AppState>) -> impl Responder {
    let mut conn: DatabaseConn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    let result = web::block(move || watchlist_item::WatchlistItem::get_all(&mut conn))
        .await
        .map_err(|err| format!("Error executing database query: {}", err))
        .and_then(|r| r.map_err(|e| e.to_string()));

    match result {
        Ok(items) => ApiResponse::success(json!(items)),
        Err(error_message) => ApiResponse::error(&error_message),
    }
}

async fn create(pool: web::Data<AppState>, item: web::Json<WatchlistItemCreateDTO>) -> impl Responder {
    let mut conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    let result = web::block(move || {
        watchlist_item::WatchlistItem::create(&mut conn, &item)
    })
    .await
    .map_err(|err| format!("Error executing database query: {}", err))
    .and_then(|r| r.map_err(|e| e.to_string()));

    match result {
        Ok(content) => ApiResponse::success(json!(content)),
        Err(err) => ApiResponse::error(&err)
    }
}
