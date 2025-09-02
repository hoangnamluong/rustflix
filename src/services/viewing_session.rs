use actix_web::{web, error::ErrorInternalServerError};
use crate::{
    config::db_config::DatabaseConn,
    models::viewing_session::{ViewingSession, ViewingSessionCreateDTO, ViewingSessionUpdateDTO},
    RepoResult,
};

pub async fn get_all(mut conn: DatabaseConn) -> RepoResult<Vec<ViewingSession>> {
    web::block(move || {        
        ViewingSession::get_all(&mut conn)
    })
    .await
    .map_err(ErrorInternalServerError)
    .and_then(|result| result.map_err(ErrorInternalServerError))
}

pub async fn get_by_id(mut conn: DatabaseConn, id: i32) -> RepoResult<ViewingSession> {
    web::block(move || {        
        ViewingSession::get_by_id(&mut conn, id)
    })
    .await
    .map_err(ErrorInternalServerError)
    .and_then(|result| result.map_err(ErrorInternalServerError))
}

pub async fn get_by_user_and_asset(mut conn: DatabaseConn, user_id: i32, asset_id: i32) -> RepoResult<ViewingSession> {
    web::block(move || {        
        ViewingSession::get_by_user_and_asset(&mut conn, user_id, asset_id)
    })
    .await
    .map_err(ErrorInternalServerError)
    .and_then(|result| result.map_err(ErrorInternalServerError))
}

pub async fn create(mut conn: DatabaseConn, session: ViewingSessionCreateDTO) -> RepoResult<usize> {
    web::block(move || {        
        ViewingSession::create(&mut conn, &session)
    })
    .await
    .map_err(ErrorInternalServerError)
    .and_then(|result| result.map_err(ErrorInternalServerError))
}

pub async fn update(mut conn: DatabaseConn, id: i32, session: ViewingSessionUpdateDTO) -> RepoResult<usize> {
    web::block(move || {        
        ViewingSession::update(&mut conn, id, &session)
    })
    .await
    .map_err(ErrorInternalServerError)
    .and_then(|result| result.map_err(ErrorInternalServerError))
}

pub async fn delete(mut conn: DatabaseConn, id: i32) -> RepoResult<usize> {
    web::block(move || {        
        ViewingSession::delete(&mut conn, id)
    })
    .await
    .map_err(ErrorInternalServerError)
    .and_then(|result| result.map_err(ErrorInternalServerError))
}
