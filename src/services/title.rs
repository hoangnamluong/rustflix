use actix_web::{web, error::ErrorInternalServerError};
use crate::{
    config::db_config::DatabaseConn,
    models::title::{Title, TitleCreateDTO, TitleUpdateDTO}, RepoResult,
};

pub async fn get_all(mut conn: DatabaseConn) -> RepoResult<Vec<Title>> {
    web::block(move || {        
        Title::get_all(&mut conn)
    })
        .await
        .map_err(ErrorInternalServerError)
        .and_then(|result| result.map_err(ErrorInternalServerError))
}

pub async fn create(mut conn: DatabaseConn, title: TitleCreateDTO) -> RepoResult<usize> {
    web::block(move || {        
        Title::create(&mut conn, &title)
    })
        .await
        .map_err(ErrorInternalServerError)
        .and_then(|result| result.map_err(ErrorInternalServerError))
}

pub async fn update(mut conn: DatabaseConn, id: i32, title: TitleUpdateDTO) -> RepoResult<usize> {
    web::block(move || {        
        Title::update(&mut conn, id, &title)
    })
    .await
    .map_err(ErrorInternalServerError)
    .and_then(|result| result.map_err(ErrorInternalServerError))
}

pub async fn delete(mut conn: DatabaseConn, id: i32) -> RepoResult<usize> {
    web::block(move || {        
        Title::delete(&mut conn, id)
    })
    .await
    .map_err(ErrorInternalServerError)
    .and_then(|result| result.map_err(ErrorInternalServerError))
}