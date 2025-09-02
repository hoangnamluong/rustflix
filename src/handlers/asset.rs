use actix_web::{ web, Responder };
use serde_json::json;
use crate::{
    app_state::AppState,
    models::asset::{AssetCreateDTO, AssetUpdateDTO},
    utils::api_response::ApiResponse
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/assets")
            .route(web::get().to(get_all))
            .route(web::post().to(create))
    )
    .service(
        web::resource("/assets/{id}")
            .route(web::get().to(get_by_id))
            .route(web::patch().to(update))
            .route(web::delete().to(delete))
    )
    .service(
        web::resource("/assets/title/{title_id}")
            .route(web::get().to(get_by_title))
    );
}

async fn get_all(pool: web::Data<AppState>) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    match crate::services::asset::get_all(conn).await {
        Ok(assets) => ApiResponse::success(json!(assets)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}

async fn get_by_id(pool: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    let id = path.into_inner();
    match crate::services::asset::get_by_id(conn, id).await {
        Ok(asset) => ApiResponse::success(json!(asset)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}

async fn get_by_title(pool: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    let title_id = path.into_inner();
    match crate::services::asset::get_by_title(conn, title_id).await {
        Ok(assets) => ApiResponse::success(json!(assets)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}

async fn create(pool: web::Data<AppState>, asset: web::Json<AssetCreateDTO>) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    match crate::services::asset::create(conn, asset.into_inner()).await {
        Ok(result) => ApiResponse::success(json!(result)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}

async fn update(
    pool: web::Data<AppState>, 
    path: web::Path<i32>,
    asset: web::Json<AssetUpdateDTO>
) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    let id = path.into_inner();
    match crate::services::asset::update(conn, id, asset.into_inner()).await {
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
    match crate::services::asset::delete(conn, id).await {
        Ok(result) => ApiResponse::success(json!(result)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}
