use actix_web::{ web, Responder };
use serde_json::json;
use crate::{
    app_state::AppState,
    models::genre::{GenreCreateDTO, GenreUpdateDTO},
    utils::api_response::ApiResponse
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/genres")
            .route(web::get().to(get_all))
            .route(web::post().to(create))
    )
    .service(
        web::resource("/genres/{id}")
            .route(web::get().to(get_by_id))
            .route(web::patch().to(update))
            .route(web::delete().to(delete))
    );
}

async fn get_all(pool: web::Data<AppState>) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    match crate::services::genre::get_all(conn).await {
        Ok(genres) => ApiResponse::success(json!(genres)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}

async fn get_by_id(pool: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    let id = path.into_inner();
    match crate::services::genre::get_by_id(conn, id).await {
        Ok(genre) => ApiResponse::success(json!(genre)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}

async fn create(pool: web::Data<AppState>, genre: web::Json<GenreCreateDTO>) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    match crate::services::genre::create(conn, genre.into_inner()).await {
        Ok(result) => ApiResponse::success(json!(result)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}

async fn update(
    pool: web::Data<AppState>,
    path: web::Path<i32>,
    genre: web::Json<GenreUpdateDTO>
) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    let id = path.into_inner();
    match crate::services::genre::update(conn, id, genre.into_inner()).await {
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
    match crate::services::genre::delete(conn, id).await {
        Ok(result) => ApiResponse::success(json!(result)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}
