use actix_web::{ web, Responder };
use serde_json::json;
use crate::{
    app_state::AppState,
    models::rating::{RatingCreateDTO, RatingUpdateDTO},
    utils::api_response::ApiResponse
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/ratings")
            .route(web::get().to(get_all))
            .route(web::post().to(create))
    )
    .service(
        web::resource("/ratings/{id}")
            .route(web::get().to(get_by_id))
            .route(web::patch().to(update))
            .route(web::delete().to(delete))
    )
    // .service(
    //     web::resource("/ratings/user/{user_id}")
    //         .route(web::get().to(get_by_user))
    // )
    .service(
        web::resource("/ratings/title/{title_id}")
            .route(web::get().to(get_by_title))
    )
    .service(
        web::resource("/ratings/user/{user_id}/title/{title_id}")
            .route(web::get().to(get_by_user_and_title))
    );
}

async fn get_all(pool: web::Data<AppState>) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    match crate::services::rating::get_all(conn).await {
        Ok(ratings) => ApiResponse::success(json!(ratings)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}

async fn get_by_id(pool: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    let id = path.into_inner();
    match crate::services::rating::get_by_id(conn, id).await {
        Ok(rating) => ApiResponse::success(json!(rating)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}

// Don't have a plan for this at the moment
// async fn get_by_user(pool: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
//     let conn = match pool.db.get() {
//         Ok(conn) => conn,
//         Err(_) => return ApiResponse::error("Failed to connect to database"),
//     };

//     let user_id = path.into_inner();
//     match crate::services::rating::get_by_user(conn, user_id).await {
//         Ok(ratings) => ApiResponse::success(json!(ratings)),
//         Err(e) => ApiResponse::error(&format!("Error: {}", e)),
//     }
// }

async fn get_by_title(pool: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    let title_id = path.into_inner();
    match crate::services::rating::get_by_title(conn, title_id).await {
        Ok(ratings) => ApiResponse::success(json!(ratings)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}

async fn get_by_user_and_title(pool: web::Data<AppState>, path: web::Path<(i32, i32)>) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    let (user_id, title_id) = path.into_inner();
    match crate::services::rating::get_by_user_and_title(conn, user_id, title_id).await {
        Ok(rating) => ApiResponse::success(json!(rating)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}

async fn create(pool: web::Data<AppState>, rating: web::Json<RatingCreateDTO>) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    match crate::services::rating::create(conn, rating.into_inner()).await {
        Ok(result) => ApiResponse::success(json!(result)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}

async fn update(
    pool: web::Data<AppState>, 
    path: web::Path<i32>,
    rating: web::Json<RatingUpdateDTO>
) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    let id = path.into_inner();
    match crate::services::rating::update(conn, id, rating.into_inner()).await {
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
    match crate::services::rating::delete(conn, id).await {
        Ok(result) => ApiResponse::success(json!(result)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}
