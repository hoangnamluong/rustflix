use actix_web::{web, error::ErrorInternalServerError};
use crate::{
    config::db_config::DatabaseConn,
    models::casting::{Casting, CastingCreateDTO, CastingUpdateDTO},
    RepoResult,
};

pub async fn get_all(mut conn: DatabaseConn) -> RepoResult<Vec<Casting>> {
    web::block(move || {        
        Casting::get_all(&mut conn)
    })
    .await
    .map_err(ErrorInternalServerError)
    .and_then(|result| result.map_err(ErrorInternalServerError))
}

pub async fn get_by_title(mut conn: DatabaseConn, title_id: i32) -> RepoResult<Vec<Casting>> {
    web::block(move || {        
        Casting::get_by_title(&mut conn, title_id)
    })
    .await
    .map_err(ErrorInternalServerError)
    .and_then(|result| result.map_err(ErrorInternalServerError))
}

pub async fn get_by_filmmaker(mut conn: DatabaseConn, filmmaker_id: i32) -> RepoResult<Vec<Casting>> {
    web::block(move || {        
        Casting::get_by_filmmaker(&mut conn, filmmaker_id)
    })
    .await
    .map_err(ErrorInternalServerError)
    .and_then(|result| result.map_err(ErrorInternalServerError))
}

pub async fn create(mut conn: DatabaseConn, casting: CastingCreateDTO) -> RepoResult<usize> {
    web::block(move || {        
        Casting::create(&mut conn, &casting)
    })
    .await
    .map_err(ErrorInternalServerError)
    .and_then(|result| result.map_err(ErrorInternalServerError))
}

pub async fn update(
    mut conn: DatabaseConn,
    title_id: i32,
    filmmaker_id: i32,
    role_id: i32,
    casting: CastingUpdateDTO
) -> RepoResult<usize> {
    web::block(move || {        
        Casting::update(&mut conn, title_id, filmmaker_id, role_id, &casting)
    })
    .await
    .map_err(ErrorInternalServerError)
    .and_then(|result| result.map_err(ErrorInternalServerError))
}

pub async fn delete(
    mut conn: DatabaseConn,
    title_id: i32,
    filmmaker_id: i32,
    role_id: i32
) -> RepoResult<usize> {
    web::block(move || {        
        Casting::delete(&mut conn, title_id, filmmaker_id, role_id)
    })
    .await
    .map_err(ErrorInternalServerError)
    .and_then(|result| result.map_err(ErrorInternalServerError))
}
