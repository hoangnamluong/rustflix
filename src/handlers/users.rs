use actix_web::{ web, Responder };
use serde_json::json;
use crate::{
    app_state::AppState,
    models::users::{UsersCreateDTO, UsersUpdateDTO},
    utils::api_response::ApiResponse
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users")
            .route(web::get().to(get_all))
            // .route(web::post().to(create))
    )
    .service(
        web::resource("/users/{id}")
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

    match crate::services::users::get_all(conn).await {
        Ok(users) => ApiResponse::success(json!(users)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}

async fn get_by_id(pool: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    let user_id = path.into_inner();
    match crate::services::users::get_by_id(conn, user_id).await {
        Ok(user) => ApiResponse::success(json!(user)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}

// User is created through register
// async fn create(pool: web::Data<AppState>, user: web::Json<UsersCreateDTO>) -> impl Responder {
//     let conn = match pool.db.get() {
//         Ok(conn) => conn,
//         Err(_) => return ApiResponse::error("Failed to connect to database"),
//     };

//     match crate::services::users::create(conn, user.into_inner()).await {
//         Ok(result) => ApiResponse::success(json!(result)),
//         Err(e) => ApiResponse::error(&format!("Error: {}", e)),
//     }
// }

async fn update(
    pool: web::Data<AppState>,
    path: web::Path<i32>,
    user: web::Json<UsersUpdateDTO>
) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    let user_id = path.into_inner();
    match crate::services::users::update(conn, user_id, user.into_inner()).await {
        Ok(result) => ApiResponse::success(json!(result)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}

async fn delete(pool: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    let user_id = path.into_inner();
    match crate::services::users::delete(conn, user_id).await {
        Ok(result) => ApiResponse::success(json!(result)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}