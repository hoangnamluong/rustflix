use actix_web::{web, error::ErrorInternalServerError};
use crate::{
    config::db_config::DatabaseConn,
    models::rating::{Rating, RatingCreateDTO, RatingUpdateDTO},
    RepoResult,
};

pub async fn get_all(mut conn: DatabaseConn) -> RepoResult<Vec<Rating>> {
    web::block(move || {        
        Rating::get_all(&mut conn)
    })
    .await
    .map_err(ErrorInternalServerError)
    .and_then(|result| result.map_err(ErrorInternalServerError))
}

pub async fn get_by_id(mut conn: DatabaseConn, id: i32) -> RepoResult<Rating> {
    web::block(move || {        
        Rating::get_by_id(&mut conn, id)
    })
    .await
    .map_err(ErrorInternalServerError)
    .and_then(|result| result.map_err(ErrorInternalServerError))
}

pub async fn get_by_title(mut conn: DatabaseConn, title_id: i32) -> RepoResult<Vec<Rating>> {
    web::block(move || {        
        Rating::get_by_title(&mut conn, title_id)
    })
    .await
    .map_err(ErrorInternalServerError)
    .and_then(|result| result.map_err(ErrorInternalServerError))
}
pub async fn get_by_user_and_title(mut conn: DatabaseConn, user_id: i32, title_id: i32) -> RepoResult<Vec<Rating>> {
    web::block(move || {
        Rating::get_by_user_and_title(&mut conn, user_id, title_id)
    })
    .await
    .map_err(ErrorInternalServerError)
    .and_then(|result| result.map_err(ErrorInternalServerError))
}

pub async fn create(mut conn: DatabaseConn, rating: RatingCreateDTO) -> RepoResult<usize> {
    web::block(move || {        
        Rating::create(&mut conn, &rating)
    })
    .await
    .map_err(ErrorInternalServerError)
    .and_then(|result| result.map_err(ErrorInternalServerError))
}

pub async fn update(mut conn: DatabaseConn, id: i32, rating: RatingUpdateDTO) -> RepoResult<usize> {
    web::block(move || {        
        Rating::update(&mut conn, id, &rating)
    })
    .await
    .map_err(ErrorInternalServerError)
    .and_then(|result| result.map_err(ErrorInternalServerError))
}

pub async fn delete(mut conn: DatabaseConn, id: i32) -> RepoResult<usize> {
    web::block(move || {        
        Rating::delete(&mut conn, id)
    })
    .await
    .map_err(ErrorInternalServerError)
    .and_then(|result| result.map_err(ErrorInternalServerError))
}
