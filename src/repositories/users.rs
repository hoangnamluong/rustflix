use diesel::dsl::insert_into;
use diesel::{Insertable, QueryResult, RunQueryDsl};
use crate::config::db_config::DatabaseConn;
use crate::models::users::{Users, UsersDTO};
use crate::schema::users;

impl Users {
    pub fn get_all(conn: &mut DatabaseConn) -> QueryResult<Vec<Self>> {
        users::table.load::<Users>(conn)
    }

    pub fn create(conn: &mut DatabaseConn, user: &UsersDTO) -> QueryResult<usize> {
        insert_into(users::dsl::users).values(user).execute(conn)
    }
}