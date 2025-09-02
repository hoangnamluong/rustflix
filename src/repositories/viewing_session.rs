use diesel::dsl::{delete, insert_into, update};
use diesel::query_dsl::methods::{FilterDsl, FindDsl};
use diesel::{ExpressionMethods, QueryResult, RunQueryDsl};
use crate::config::db_config::DatabaseConn;
use crate::models::viewing_session::{ViewingSession, ViewingSessionCreateDTO, ViewingSessionUpdateDTO};
use crate::schema::viewing_session::{self, dsl};

impl ViewingSession {
    pub fn get_all(conn: &mut DatabaseConn) -> QueryResult<Vec<Self>> {
        viewing_session::table.load::<ViewingSession>(conn)
    }

    pub fn get_by_id(conn: &mut DatabaseConn, id: i32) -> QueryResult<Self> {
        viewing_session::table.filter(dsl::id.eq(id)).first(conn)
    }

    // Viewing Sessions need to be filter with user and asset
    // pub fn get_by_user(conn: &mut DatabaseConn, user_id: i32) -> QueryResult<Vec<Self>> {
    //     viewing_session::table.filter(dsl::user_id.eq(user_id)).load(conn)
    // }

    // pub fn get_by_asset(conn: &mut DatabaseConn, asset_id: i32) -> QueryResult<Vec<Self>> {
    //     viewing_session::table.filter(dsl::asset_id.eq(asset_id)).load(conn)
    // }

    pub fn get_by_user_and_asset(conn: &mut DatabaseConn, user_id: i32, asset_id: i32) -> QueryResult<Self> {
        viewing_session::table
            .filter(dsl::user_id.eq(user_id))
            .filter(dsl::asset_id.eq(asset_id))
            .first(conn)
    }

    pub fn create(conn: &mut DatabaseConn, session: &ViewingSessionCreateDTO) -> QueryResult<usize> {
        insert_into(viewing_session::dsl::viewing_session).values(session).execute(conn)
    }

    pub fn update(conn: &mut DatabaseConn, id: i32, session: &ViewingSessionUpdateDTO) -> QueryResult<usize> {
        update(dsl::viewing_session.find(id)).set(session).execute(conn)
    }

    pub fn delete(conn: &mut DatabaseConn, id: i32) -> QueryResult<usize> {
        delete(dsl::viewing_session.find(id)).execute(conn)
    }
}
