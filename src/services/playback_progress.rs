use actix_web::{web, error::ErrorInternalServerError};
use crate::{
    config::db_config::DatabaseConn,
    models::playback_progress::{PlaybackProgress, PlaybackProgressCreateDTO, PlaybackProgressUpdateDTO},
    RepoResult,
};

pub async fn get_all(mut conn: DatabaseConn) -> RepoResult<Vec<PlaybackProgress>> {
    web::block(move || {        
        PlaybackProgress::get_all(&mut conn)
    })
    .await
    .map_err(ErrorInternalServerError)
    .and_then(|result| result.map_err(ErrorInternalServerError))
}

pub async fn get_by_user_and_asset(mut conn: DatabaseConn, user_id: i32, asset_id: i32) -> RepoResult<PlaybackProgress> {
    web::block(move || {        
        PlaybackProgress::get_by_user_and_asset(&mut conn, user_id, asset_id)
    })
    .await
    .map_err(ErrorInternalServerError)
    .and_then(|result| result.map_err(ErrorInternalServerError))
}

pub async fn get_by_user(mut conn: DatabaseConn, user_id: i32) -> RepoResult<Vec<PlaybackProgress>> {
    web::block(move || {        
        PlaybackProgress::get_by_user(&mut conn, user_id)
    })
    .await
    .map_err(ErrorInternalServerError)
    .and_then(|result| result.map_err(ErrorInternalServerError))
}

pub async fn get_by_asset(mut conn: DatabaseConn, asset_id: i32) -> RepoResult<Vec<PlaybackProgress>> {
    web::block(move || {        
        PlaybackProgress::get_by_asset(&mut conn, asset_id)
    })
    .await
    .map_err(ErrorInternalServerError)
    .and_then(|result| result.map_err(ErrorInternalServerError))
}

pub async fn create(mut conn: DatabaseConn, progress: PlaybackProgressCreateDTO) -> RepoResult<usize> {
    web::block(move || {        
        PlaybackProgress::create(&mut conn, &progress)
    })
    .await
    .map_err(ErrorInternalServerError)
    .and_then(|result| result.map_err(ErrorInternalServerError))
}

pub async fn update(mut conn: DatabaseConn, user_id: i32, asset_id: i32, progress: PlaybackProgressUpdateDTO) -> RepoResult<usize> {
    web::block(move || {        
        PlaybackProgress::update(&mut conn, user_id, asset_id, &progress)
    })
    .await
    .map_err(ErrorInternalServerError)
    .and_then(|result| result.map_err(ErrorInternalServerError))
}

pub async fn delete(mut conn: DatabaseConn, user_id: i32, asset_id: i32) -> RepoResult<usize> {
    web::block(move || {        
        PlaybackProgress::delete(&mut conn, user_id, asset_id)
    })
    .await
    .map_err(ErrorInternalServerError)
    .and_then(|result| result.map_err(ErrorInternalServerError))
}
