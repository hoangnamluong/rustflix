use diesel::dsl::{delete, insert_into, update};
use diesel::query_dsl::methods::{FilterDsl, FindDsl};
use diesel::{ExpressionMethods, QueryResult, RunQueryDsl};
use crate::config::db_config::DatabaseConn;
use crate::models::video_file::{VideoFile, VideoFileCreateDTO, VideoFileUpdateDTO};
use crate::schema::video_file::{self, dsl};

impl VideoFile {
    pub fn get_all(conn: &mut DatabaseConn) -> QueryResult<Vec<Self>> {
        video_file::table.load::<VideoFile>(conn)
    }

    pub fn get_by_id(conn: &mut DatabaseConn, id: i32) -> QueryResult<Self> {
        video_file::table.filter(dsl::id.eq(id)).first(conn)
    }

    pub fn get_by_asset(conn: &mut DatabaseConn, asset_id: i32) -> QueryResult<Vec<Self>> {
        video_file::table.filter(dsl::asset_id.eq(asset_id)).load::<VideoFile>(conn)
    }

    pub fn create(conn: &mut DatabaseConn, file: &VideoFileCreateDTO) -> QueryResult<usize> {
        insert_into(video_file::dsl::video_file).values(file).execute(conn)
    }

    pub fn update(conn: &mut DatabaseConn, id: i32, file: &VideoFileUpdateDTO) -> QueryResult<usize> {
        update(dsl::video_file.find(id)).set(file).execute(conn)
    }

    pub fn delete(conn: &mut DatabaseConn, id: i32) -> QueryResult<usize> {
        delete(dsl::video_file.find(id)).execute(conn)
    }
}
