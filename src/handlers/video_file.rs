use actix_web::{ web, Responder };
use serde_json::json;
use crate::{
    app_state::AppState,
    models::video_file::{VideoFileCreateDTO, VideoFileUpdateDTO},
    utils::api_response::ApiResponse
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/video-files")
            .route(web::get().to(get_all))
            .route(web::post().to(create))
    )
    .service(
        web::resource("/video-files/{id}")
            .route(web::get().to(get_by_id))
            .route(web::patch().to(update))
            .route(web::delete().to(delete))
    )
    .service(
        web::resource("/video-files/asset/{asset_id}")
            .route(web::get().to(get_by_asset))
    );
}

async fn get_all(pool: web::Data<AppState>) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    match crate::services::video_file::get_all(conn).await {
        Ok(files) => ApiResponse::success(json!(files)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}

async fn get_by_id(pool: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    let id = path.into_inner();
    match crate::services::video_file::get_by_id(conn, id).await {
        Ok(file) => ApiResponse::success(json!(file)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}

async fn get_by_asset(pool: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    let asset_id = path.into_inner();
    match crate::services::video_file::get_by_asset(conn, asset_id).await {
        Ok(files) => ApiResponse::success(json!(files)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}

async fn create(pool: web::Data<AppState>, file: web::Json<VideoFileCreateDTO>) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    match crate::services::video_file::create(conn, file.into_inner()).await {
        Ok(result) => ApiResponse::success(json!(result)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}

async fn update(
    pool: web::Data<AppState>, 
    path: web::Path<i32>,
    file: web::Json<VideoFileUpdateDTO>
) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    let id = path.into_inner();
    match crate::services::video_file::update(conn, id, file.into_inner()).await {
        Ok(result) => ApiResponse::success(json!(result)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}

async fn delete(pool: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    let id = path.into_inner();
    match crate::services::video_file::delete(conn, id).await {
        Ok(result) => ApiResponse::success(json!(result)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}
