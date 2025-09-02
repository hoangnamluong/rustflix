use actix_web::{ web, Responder };
use serde_json::json;
use crate::{
    app_state::AppState,
    models::casting_role::{CastingRoleCreateDTO, CastingRoleUpdateDTO},
    utils::api_response::ApiResponse
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/casting-roles")
            .route(web::get().to(get_all))
            .route(web::post().to(create))
    )
    .service(
        web::resource("/casting-roles/{id}")
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

    match crate::services::casting_role::get_all(conn).await {
        Ok(roles) => ApiResponse::success(json!(roles)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}

async fn get_by_id(pool: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    let id = path.into_inner();
    match crate::services::casting_role::get_by_id(conn, id).await {
        Ok(role) => ApiResponse::success(json!(role)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}

async fn create(pool: web::Data<AppState>, role: web::Json<CastingRoleCreateDTO>) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    match crate::services::casting_role::create(conn, role.into_inner()).await {
        Ok(result) => ApiResponse::success(json!(result)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}

async fn update(
    pool: web::Data<AppState>, 
    path: web::Path<i32>,
    role: web::Json<CastingRoleUpdateDTO>
) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    let id = path.into_inner();
    match crate::services::casting_role::update(conn, id, role.into_inner()).await {
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
    match crate::services::casting_role::delete(conn, id).await {
        Ok(result) => ApiResponse::success(json!(result)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}
