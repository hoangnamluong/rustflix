use diesel::dsl::{delete, insert_into, update};
use diesel::query_dsl::methods::{FilterDsl, FindDsl};
use diesel::{ExpressionMethods, QueryResult, RunQueryDsl};
use crate::config::db_config::DatabaseConn;
use crate::models::playback_progress::{PlaybackProgress, PlaybackProgressCreateDTO, PlaybackProgressUpdateDTO};
use crate::schema::playback_progress::{self, dsl};

impl PlaybackProgress {
    pub fn get_all(conn: &mut DatabaseConn) -> QueryResult<Vec<Self>> {
        playback_progress::table.load::<PlaybackProgress>(conn)
    }

    pub fn get_by_user_and_asset(conn: &mut DatabaseConn, user_id: i32, asset_id: i32) -> QueryResult<Self> {
        playback_progress::table
            .filter(dsl::user_id.eq(user_id))
            .filter(dsl::asset_id.eq(asset_id))
            .first(conn)
    }

    pub fn get_by_user(conn: &mut DatabaseConn, user_id: i32) -> QueryResult<Vec<Self>> {
        playback_progress::table.filter(dsl::user_id.eq(user_id)).load::<PlaybackProgress>(conn)
    }

    pub fn get_by_asset(conn: &mut DatabaseConn, asset_id: i32) -> QueryResult<Vec<Self>> {
        playback_progress::table.filter(dsl::asset_id.eq(asset_id)).load::<PlaybackProgress>(conn)
    }

    pub fn create(conn: &mut DatabaseConn, progress: &PlaybackProgressCreateDTO) -> QueryResult<usize> {
        insert_into(playback_progress::dsl::playback_progress).values(progress).execute(conn)
    }

    pub fn update(conn: &mut DatabaseConn, user_id: i32, asset_id: i32, progress: &PlaybackProgressUpdateDTO) -> QueryResult<usize> {
        update(playback_progress::table
            .filter(dsl::user_id.eq(user_id))
            .filter(dsl::asset_id.eq(asset_id)))
            .set(progress)
            .execute(conn)
    }

    pub fn delete(conn: &mut DatabaseConn, user_id: i32, asset_id: i32) -> QueryResult<usize> {
        delete(playback_progress::table
            .filter(dsl::user_id.eq(user_id))
            .filter(dsl::asset_id.eq(asset_id)))
            .execute(conn)
    }
}
