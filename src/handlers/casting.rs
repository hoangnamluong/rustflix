use actix_web::{ web, Responder };
use serde_json::json;
use crate::{
    app_state::AppState,
    models::casting::{CastingCreateDTO, CastingUpdateDTO},
    utils::api_response::ApiResponse
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/castings")
            .route(web::get().to(get_all))
            .route(web::post().to(create))
    )
    .service(
        web::resource("/castings/title/{title_id}")
            .route(web::get().to(get_by_title))
    )
    .service(
        web::resource("/castings/filmmaker/{filmmaker_id}")
            .route(web::get().to(get_by_filmmaker))
    )
    .service(
        web::resource("/castings/{title_id}/{filmmaker_id}/{role_id}")
            .route(web::patch().to(update))
            .route(web::delete().to(delete))
    );
}

async fn get_all(pool: web::Data<AppState>) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    match crate::services::casting::get_all(conn).await {
        Ok(castings) => ApiResponse::success(json!(castings)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}

async fn get_by_title(pool: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    let title_id = path.into_inner();
    match crate::services::casting::get_by_title(conn, title_id).await {
        Ok(castings) => ApiResponse::success(json!(castings)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}

async fn get_by_filmmaker(pool: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    let filmmaker_id = path.into_inner();
    match crate::services::casting::get_by_filmmaker(conn, filmmaker_id).await {
        Ok(castings) => ApiResponse::success(json!(castings)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}

async fn create(pool: web::Data<AppState>, casting: web::Json<CastingCreateDTO>) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    match crate::services::casting::create(conn, casting.into_inner()).await {
        Ok(result) => ApiResponse::success(json!(result)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}

#[derive(serde::Deserialize)]
pub struct CastingIds {
    title_id: i32,
    filmmaker_id: i32,
    role_id: i32,
}

async fn update(
    pool: web::Data<AppState>,
    path: web::Path<CastingIds>,
    casting: web::Json<CastingUpdateDTO>
) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    let ids = path.into_inner();
    match crate::services::casting::update(
        conn,
        ids.title_id,
        ids.filmmaker_id,
        ids.role_id,
        casting.into_inner()
    ).await {
        Ok(result) => ApiResponse::success(json!(result)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}

async fn delete(pool: web::Data<AppState>, path: web::Path<CastingIds>) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    let ids = path.into_inner();
    match crate::services::casting::delete(conn, ids.title_id, ids.filmmaker_id, ids.role_id).await {
        Ok(result) => ApiResponse::success(json!(result)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}
