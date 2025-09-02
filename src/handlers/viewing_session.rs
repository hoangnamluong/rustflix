use actix_web::{ web, Responder };
use serde_json::json;
use crate::{
    app_state::AppState,
    models::viewing_session::{ViewingSessionCreateDTO, ViewingSessionUpdateDTO},
    utils::api_response::ApiResponse
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/viewing-sessions")
            .route(web::get().to(get_all))
            .route(web::post().to(create))
    )
    .service(
        web::resource("/viewing-sessions/{id}")
            .route(web::get().to(get_by_id))
            .route(web::patch().to(update))
            .route(web::delete().to(delete))
    )
    .service(
        web::resource("/viewing-sessions/asset/{asset_id}/user/{user_id}")
            .route(web::get().to(get_by_user_and_asset))
    );
    // .service(
    //     web::resource("/viewing-sessions/user/{user_id}")
    //         .route(web::get().to(get_by_user))
    // )
    // .service(
    //     web::resource("/viewing-sessions/asset/{asset_id}")
    //         .route(web::get().to(get_by_asset))
    // );
}

async fn get_all(pool: web::Data<AppState>) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    match crate::services::viewing_session::get_all(conn).await {
        Ok(sessions) => ApiResponse::success(json!(sessions)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}

async fn get_by_id(pool: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    let id = path.into_inner();
    match crate::services::viewing_session::get_by_id(conn, id).await {
        Ok(session) => ApiResponse::success(json!(session)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}

// Viewing Sessions need to be filter with user and asset
// async fn get_by_user(pool: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
//     let conn = match pool.db.get() {
//         Ok(conn) => conn,
//         Err(_) => return ApiResponse::error("Failed to connect to database"),
//     };

//     let user_id = path.into_inner();
//     match crate::services::viewing_session::get_by_user(conn, user_id).await {
//         Ok(sessions) => ApiResponse::success(json!(sessions)),
//         Err(e) => ApiResponse::error(&format!("Error: {}", e)),
//     }
// }

// async fn get_by_asset(pool: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
//     let conn = match pool.db.get() {
//         Ok(conn) => conn,
//         Err(_) => return ApiResponse::error("Failed to connect to database"),
//     };

//     let asset_id = path.into_inner();
//     match crate::services::viewing_session::get_by_asset(conn, asset_id).await {
//         Ok(sessions) => ApiResponse::success(json!(sessions)),
//         Err(e) => ApiResponse::error(&format!("Error: {}", e)),
//     }
// }

async fn get_by_user_and_asset(pool: web::Data<AppState>, path: web::Path<(i32, i32)>) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    let (asset_id, user_id) = path.into_inner();
    match crate::services::viewing_session::get_by_user_and_asset(conn, user_id, asset_id).await {
        Ok(session) => ApiResponse::success(json!(session)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}

async fn create(pool: web::Data<AppState>, session: web::Json<ViewingSessionCreateDTO>) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    match crate::services::viewing_session::create(conn, session.into_inner()).await {
        Ok(result) => ApiResponse::success(json!(result)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}

async fn update(
    pool: web::Data<AppState>, 
    path: web::Path<i32>,
    session: web::Json<ViewingSessionUpdateDTO>
) -> impl Responder {
    let conn = match pool.db.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error("Failed to connect to database"),
    };

    let id = path.into_inner();
    match crate::services::viewing_session::update(conn, id, session.into_inner()).await {
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
    match crate::services::viewing_session::delete(conn, id).await {
        Ok(result) => ApiResponse::success(json!(result)),
        Err(e) => ApiResponse::error(&format!("Error: {}", e)),
    }
}
