use actix_web::{ web, Responder };
use serde_json::json;
use crate::{
    app_state::AppState,
    models::playback_progress::{PlaybackProgressCreateDTO, PlaybackProgressUpdateDTO},
    utils::api_response::ApiResponse
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/playback-progress")
            .route(web::get().to(get_all))
            .route(web::post().to(create))
    )
    .service(
        web::resource("/playback-progress/user/{user_id}")
            .route(web::get().to(get_by_user))
    )
    .service(
        web::resource("/playback-progress/asset/{asset_id}")
            .route(web::get().to(get_by_asset))
    )
    .service(
        web::resource("/playback-progress/{user_id}/{asset_id}")
            .route(web::get().to(get_by_user_and_asset))
            .route(web::patch().to(update))
            .route(web::delete().to(delete))
    );
}

async fn get_all(pool: web::Data<AppState>) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    match crate::services::playback_progress::get_all(conn).await {
        Ok(progress) => ApiResponse::success(json!(progress)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}

async fn get_by_user(pool: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    let user_id = path.into_inner();
    match crate::services::playback_progress::get_by_user(conn, user_id).await {
        Ok(progress) => ApiResponse::success(json!(progress)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}

async fn get_by_asset(pool: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    let asset_id = path.into_inner();
    match crate::services::playback_progress::get_by_asset(conn, asset_id).await {
        Ok(progress) => ApiResponse::success(json!(progress)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}

#[derive(serde::Deserialize)]
pub struct UserAssetIds {
    user_id: i32,
    asset_id: i32,
}

async fn get_by_user_and_asset(pool: web::Data<AppState>, path: web::Path<UserAssetIds>) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    let ids = path.into_inner();
    match crate::services::playback_progress::get_by_user_and_asset(conn, ids.user_id, ids.asset_id).await {
        Ok(progress) => ApiResponse::success(json!(progress)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}

async fn create(pool: web::Data<AppState>, progress: web::Json<PlaybackProgressCreateDTO>) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    match crate::services::playback_progress::create(conn, progress.into_inner()).await {
        Ok(result) => ApiResponse::success(json!(result)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}

async fn update(
    pool: web::Data<AppState>, 
    path: web::Path<UserAssetIds>,
    progress: web::Json<PlaybackProgressUpdateDTO>
) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    let ids = path.into_inner();
    match crate::services::playback_progress::update(conn, ids.user_id, ids.asset_id, progress.into_inner()).await {
        Ok(result) => ApiResponse::success(json!(result)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}

async fn delete(pool: web::Data<AppState>, path: web::Path<UserAssetIds>) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    let ids = path.into_inner();
    match crate::services::playback_progress::delete(conn, ids.user_id, ids.asset_id).await {
        Ok(result) => ApiResponse::success(json!(result)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}
