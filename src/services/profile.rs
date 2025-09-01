use actix_web::{web, error::ErrorInternalServerError};
use crate::{
    config::db_config::DatabaseConn,
    models::profile::{Profile, ProfileCreateDTO, ProfileUpdateDTO},
    RepoResult,
};

pub async fn get_all(mut conn: DatabaseConn) -> RepoResult<Vec<Profile>> {
    web::block(move || {        
        Profile::get_all(&mut conn)
    })
    .await
    .map_err(ErrorInternalServerError)
    .and_then(|result| result.map_err(ErrorInternalServerError))
}

pub async fn get_by_id(mut conn: DatabaseConn, id: i32) -> RepoResult<Profile> {
    web::block(move || {        
        Profile::get_by_id(&mut conn, id)
    })
    .await
    .map_err(ErrorInternalServerError)
    .and_then(|result| result.map_err(ErrorInternalServerError))
}

pub async fn get_by_user_id(mut conn: DatabaseConn, user_id: i32) -> RepoResult<Vec<Profile>> {
    web::block(move || {        
        Profile::get_by_user_id(&mut conn, user_id)
    })
    .await
    .map_err(ErrorInternalServerError)
    .and_then(|result| result.map_err(ErrorInternalServerError))
}

pub async fn create(mut conn: DatabaseConn, profile: ProfileCreateDTO) -> RepoResult<usize> {
    web::block(move || {        
        Profile::create(&mut conn, &profile)
    })
    .await
    .map_err(ErrorInternalServerError)
    .and_then(|result| result.map_err(ErrorInternalServerError))
}

pub async fn update(mut conn: DatabaseConn, id: i32, profile: ProfileUpdateDTO) -> RepoResult<usize> {
    web::block(move || {        
        Profile::update(&mut conn, id, &profile)
    })
    .await
    .map_err(ErrorInternalServerError)
    .and_then(|result| result.map_err(ErrorInternalServerError))
}

pub async fn delete(mut conn: DatabaseConn, id: i32) -> RepoResult<usize> {
    web::block(move || {        
        Profile::delete(&mut conn, id)
    })
    .await
    .map_err(ErrorInternalServerError)
    .and_then(|result| result.map_err(ErrorInternalServerError))
}
