use actix_web::{ web, Responder };
use serde_json::json;
use crate::{
    app_state::AppState,
    config::db_config::DatabaseConn,
    models::video_file::{self, VideoFileCreateDTO},
    utils::api_response::ApiResponse
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/video-files")
            .route(web::get().to(get_all))
            .route(web::post().to(create))
    );
}

async fn get_all(pool: web::Data<AppState>) -> impl Responder {
    let mut conn: DatabaseConn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    let result = web::block(move || video_file::VideoFile::get_all(&mut conn))
        .await
        .map_err(|err| format!("Error executing database query: {}", err))
        .and_then(|r| r.map_err(|e| e.to_string()));

    match result {
        Ok(files) => ApiResponse::success(json!(files)),
        Err(error_message) => ApiResponse::error(&error_message),
    }
}

async fn create(pool: web::Data<AppState>, file: web::Json<VideoFileCreateDTO>) -> impl Responder {
    let mut conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    let result = web::block(move || {
        video_file::VideoFile::create(&mut conn, &file)
    })
    .await
    .map_err(|err| format!("Error executing database query: {}", err))
    .and_then(|r| r.map_err(|e| e.to_string()));

    match result {
        Ok(content) => ApiResponse::success(json!(content)),
        Err(err) => ApiResponse::error(&err)
    }
}
