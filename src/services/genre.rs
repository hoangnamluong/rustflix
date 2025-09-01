use actix_web::{web, error::ErrorInternalServerError};
use crate::{
    config::db_config::DatabaseConn,
    models::genre::{Genre, GenreCreateDTO, GenreUpdateDTO},
    RepoResult,
};

pub async fn get_all(mut conn: DatabaseConn) -> RepoResult<Vec<Genre>> {
    web::block(move || {        
        Genre::get_all(&mut conn)
    })
    .await
    .map_err(ErrorInternalServerError)
    .and_then(|result| result.map_err(ErrorInternalServerError))
}

pub async fn get_by_id(mut conn: DatabaseConn, id: i32) -> RepoResult<Genre> {
    web::block(move || {        
        Genre::get_by_id(&mut conn, id)
    })
    .await
    .map_err(ErrorInternalServerError)
    .and_then(|result| result.map_err(ErrorInternalServerError))
}

pub async fn create(mut conn: DatabaseConn, genre: GenreCreateDTO) -> RepoResult<usize> {
    web::block(move || {        
        Genre::create(&mut conn, &genre)
    })
    .await
    .map_err(ErrorInternalServerError)
    .and_then(|result| result.map_err(ErrorInternalServerError))
}

pub async fn update(mut conn: DatabaseConn, id: i32, genre: GenreUpdateDTO) -> RepoResult<usize> {
    web::block(move || {        
        Genre::update(&mut conn, id, &genre)
    })
    .await
    .map_err(ErrorInternalServerError)
    .and_then(|result| result.map_err(ErrorInternalServerError))
}

pub async fn delete(mut conn: DatabaseConn, id: i32) -> RepoResult<usize> {
    web::block(move || {        
        Genre::delete(&mut conn, id)
    })
    .await
    .map_err(ErrorInternalServerError)
    .and_then(|result| result.map_err(ErrorInternalServerError))
}
