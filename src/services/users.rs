use actix_web::{web, error::ErrorInternalServerError};
use crate::{
    config::db_config::DatabaseConn,
    models::users::{Users, UsersCreateDTO, UsersUpdateDTO},
    RepoResult,
};

pub async fn get_all(mut conn: DatabaseConn) -> RepoResult<Vec<Users>> {
    web::block(move || {        
        Users::get_all(&mut conn)
    })
    .await
    .map_err(ErrorInternalServerError)
    .and_then(|result| result.map_err(ErrorInternalServerError))
}

pub async fn get_by_id(mut conn: DatabaseConn, id: i32) -> RepoResult<Users> {
    web::block(move || {        
        Users::get_by_id(&mut conn, id)
    })
    .await
    .map_err(ErrorInternalServerError)
    .and_then(|result| result.map_err(ErrorInternalServerError))
}

// User is created through register
// pub async fn create(mut conn: DatabaseConn, user: UsersCreateDTO) -> RepoResult<usize> {
//     web::block(move || {        
//         Users::create(&mut conn, &user)
//     })
//     .await
//     .map_err(ErrorInternalServerError)
//     .and_then(|result| result.map_err(ErrorInternalServerError))
// }

pub async fn update(mut conn: DatabaseConn, id: i32, user: UsersUpdateDTO) -> RepoResult<usize> {
    web::block(move || {        
        Users::update(&mut conn, id, &user)
    })
    .await
    .map_err(ErrorInternalServerError)
    .and_then(|result| result.map_err(ErrorInternalServerError))
}

pub async fn delete(mut conn: DatabaseConn, id: i32) -> RepoResult<usize> {
    web::block(move || {        
        Users::delete(&mut conn, id)
    })
    .await
    .map_err(ErrorInternalServerError)
    .and_then(|result| result.map_err(ErrorInternalServerError))
}
