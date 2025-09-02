use actix_web::{ web, Responder };
use serde_json::json;
use crate::{
    app_state::AppState,
    models::watchlist_item::WatchlistItemCreateDTO,
    utils::api_response::ApiResponse
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/watchlist-items")
            .route(web::get().to(get_all))
            .route(web::post().to(create))
    )
    .service(
        web::resource("/watchlist-items/user/{user_id}")
            .route(web::get().to(get_by_user))
    )
    .service(
        web::resource("/watchlist-items/user/{user_id}/title/{title_id}")
            .route(web::delete().to(delete))
    );
    // .service(
    //     web::resource("/watchlist-items/title/{title_id}")
    //         .route(web::get().to(get_by_title))
    // )
    // .service(
    //     web::resource("/watchlist-items/{user_id}/{title_id}")
    //         .route(web::delete().to(delete))
    //         .route(web::get().to(get_by_user_and_title))
    // );
}

async fn get_all(pool: web::Data<AppState>) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    match crate::services::watchlist_item::get_all(conn).await {
        Ok(items) => ApiResponse::success(json!(items)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}

async fn get_by_user(pool: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    let user_id = path.into_inner();
    match crate::services::watchlist_item::get_by_user(conn, user_id).await {
        Ok(items) => ApiResponse::success(json!(items)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}

// User only see their watchlist
// async fn get_by_title(pool: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
//     let conn = match pool.db.get() {
//         Ok(conn) => conn,
//         Err(_) => return ApiResponse::error("Failed to connect to database"),
//     };

//     let title_id = path.into_inner();
//     match crate::services::watchlist_item::get_by_title(conn, title_id).await {
//         Ok(items) => ApiResponse::success(json!(items)),
//         Err(e) => ApiResponse::error(&format!("Error: {}", e)),
//     }
// }

// User only see their watchlist
// async fn get_by_user_and_title(pool: web::Data<AppState>, path: web::Path<UserTitleIds>) -> impl Responder {
//     let conn = match pool.db.get() {
//         Ok(conn) => conn,
//         Err(_) => return ApiResponse::error("Failed to connect to database"),
//     };

//     let ids = path.into_inner();
//     match crate::services::watchlist_item::get_by_user_and_title(conn, ids.user_id, ids.title_id).await {
//         Ok(item) => ApiResponse::success(json!(item)),
//         Err(e) => ApiResponse::error(&format!("Error: {}", e)),
//     }
// }

async fn create(pool: web::Data<AppState>, item: web::Json<WatchlistItemCreateDTO>) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    match crate::services::watchlist_item::create(conn, item.into_inner()).await {
        Ok(result) => ApiResponse::success(json!(result)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}

async fn delete(pool: web::Data<AppState>, path: web::Path<(i32, i32)>) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    let (user_id, title_id) = path.into_inner();
    match crate::services::watchlist_item::delete(conn, user_id, title_id).await {
        Ok(result) => ApiResponse::success(json!(result)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}
