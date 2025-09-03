use diesel::dsl::{delete, insert_into, update};
use diesel::query_dsl::methods::{FilterDsl, FindDsl};
use diesel::{ExpressionMethods, QueryResult, RunQueryDsl};
use crate::config::db_config::DatabaseConn;
use crate::models::profile::{Profile, ProfileCreateDTO, ProfileUpdateDTO};
use crate::schema::profile::{self, dsl};

impl Profile {
    pub fn get_all(conn: &mut DatabaseConn) -> QueryResult<Vec<Self>> {
        profile::table.load::<Profile>(conn)
    }

    pub fn get_by_id(conn: &mut DatabaseConn, id: i32) -> QueryResult<Self> {
        profile::table.filter(dsl::id.eq(id)).first(conn)
    }

    pub fn get_by_user_id(conn: &mut DatabaseConn, user_id: i32) -> QueryResult<Vec<Self>> {
        profile::table.filter(dsl::user_id.eq(user_id)).load::<Profile>(conn)
    }

    pub fn create(conn: &mut DatabaseConn, profile: &ProfileCreateDTO) -> QueryResult<Profile> {
        insert_into(profile::dsl::profile).values(profile).get_result(conn)
    }

    pub fn update(conn: &mut DatabaseConn, id: i32, profile: &ProfileUpdateDTO) -> QueryResult<usize> {
        update(dsl::profile.find(id)).set(profile).execute(conn)
    }

    pub fn delete(conn: &mut DatabaseConn, id: i32) -> QueryResult<usize> {
        delete(dsl::profile.find(id)).execute(conn)
    }
}
