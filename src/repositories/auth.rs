use actix_web::error::ErrorInternalServerError;
use diesel::Connection;

use crate::{config::db_config::DatabaseConn, models::{auth::RegisterDTO, profile::{Profile, ProfileCreateDTO}, users::{Users, UsersCreateDTO}}, utils::db_helper, RepoResult};

pub fn register(conn: &mut DatabaseConn, auth: &UsersCreateDTO) -> RepoResult<u8> {
    conn.transaction::<u8, diesel::result::Error, _>(|conn| {
        let user = Users::create(conn, auth)?;

        let at_idx = user.email.find("@").unwrap();
        let name = &user.email[0..at_idx]; 

        // let profile = Profile::create(conn, &ProfileCreateDTO {
        //     name: name.to_string(),

        // })?;

        Ok(1)
    })
    .map_err(ErrorInternalServerError)
}