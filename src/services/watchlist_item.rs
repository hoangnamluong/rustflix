use actix_web::{web, error::ErrorInternalServerError};
use crate::{
    config::db_config::DatabaseConn,
    models::watchlist_item::{WatchlistItem, WatchlistItemCreateDTO},
    RepoResult,
};

pub async fn get_all(mut conn: DatabaseConn) -> RepoResult<Vec<WatchlistItem>> {
    web::block(move || {        
        WatchlistItem::get_all(&mut conn)
    })
    .await
    .map_err(ErrorInternalServerError)
    .and_then(|result| result.map_err(ErrorInternalServerError))
}

pub async fn get_by_title(mut conn: DatabaseConn, title_id: i32) -> RepoResult<Vec<WatchlistItem>> {
    web::block(move || {        
        WatchlistItem::get_by_title(&mut conn, title_id)
    })
    .await
    .map_err(ErrorInternalServerError)
    .and_then(|result| result.map_err(ErrorInternalServerError))
}

pub async fn create(mut conn: DatabaseConn, item: WatchlistItemCreateDTO) -> RepoResult<usize> {
    web::block(move || {        
        WatchlistItem::create(&mut conn, &item)
    })
    .await
    .map_err(ErrorInternalServerError)
    .and_then(|result| result.map_err(ErrorInternalServerError))
}

pub async fn delete(mut conn: DatabaseConn, profile_id: i32, title_id: i32) -> RepoResult<usize> {
    web::block(move || {        
        WatchlistItem::delete(&mut conn, profile_id, title_id)
    })
    .await
    .map_err(ErrorInternalServerError)
    .and_then(|result| result.map_err(ErrorInternalServerError))
}
