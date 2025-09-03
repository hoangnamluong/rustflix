use actix_web::{ web, Responder };
use serde_json::json;
use crate::{
    app_state::AppState,
    models::profile::{ProfileCreateDTO, ProfileUpdateDTO},
    utils::api_response::ApiResponse
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/profiles")
            .route(web::get().to(get_all))
            // .route(web::post().to(create))
    )
    .service(
        web::resource("/profiles/{id}")
            .route(web::get().to(get_by_id))
            .route(web::patch().to(update))
            .route(web::delete().to(delete))
    )
    .service(
        web::resource("/users/{user_id}/profiles")
            .route(web::get().to(get_by_user_id))
    );
}

async fn get_all(pool: web::Data<AppState>) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    match crate::services::profile::get_all(conn).await {
        Ok(profiles) => ApiResponse::success(json!(profiles)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}

async fn get_by_id(pool: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    let profile_id = path.into_inner();
    match crate::services::profile::get_by_id(conn, profile_id).await {
        Ok(profile) => ApiResponse::success(json!(profile)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}

async fn get_by_user_id(pool: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    let user_id = path.into_inner();
    match crate::services::profile::get_by_user_id(conn, user_id).await {
        Ok(profiles) => ApiResponse::success(json!(profiles)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}

// Profile is created when user register
// async fn create(pool: web::Data<AppState>, profile: web::Json<ProfileCreateDTO>) -> impl Responder {
//     let conn = match pool.db.get() {
//         Ok(conn) => conn,
//         Err(_) => return ApiResponse::error("Failed to connect to database"),
//     };

//     match crate::services::profile::create(conn, profile.into_inner()).await {
//         Ok(result) => ApiResponse::success(json!(result)),
//         Err(e) => ApiResponse::error(&format!("Error: {}", e)),
//     }
// }

async fn update(
    pool: web::Data<AppState>,
    path: web::Path<i32>,
    profile: web::Json<ProfileUpdateDTO>
) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    let profile_id = path.into_inner();
    match crate::services::profile::update(conn, profile_id, profile.into_inner()).await {
        Ok(result) => ApiResponse::success(json!(result)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}

async fn delete(pool: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    let profile_id = path.into_inner();
    match crate::services::profile::delete(conn, profile_id).await {
        Ok(result) => ApiResponse::success(json!(result)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}
