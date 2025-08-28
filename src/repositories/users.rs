use diesel::{QueryResult, RunQueryDsl};
use crate::config::db_config::DatabaseConn;
use crate::models::users::Users;
use crate::schema::users;

impl Users {
    pub fn get_all(conn: &mut DatabaseConn) -> QueryResult<Vec<Self>> {
        users::table.load::<Users>(conn)
    }
}