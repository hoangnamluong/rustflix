use diesel::dsl::{delete, insert_into, update};
use diesel::query_dsl::methods::{FilterDsl, FindDsl};
use diesel::{ExpressionMethods, QueryResult, RunQueryDsl};
use crate::config::db_config::DatabaseConn;
use crate::models::asset::{Asset, AssetCreateDTO, AssetUpdateDTO};
use crate::schema::asset::{self, dsl};

impl Asset {
    pub fn get_all(conn: &mut DatabaseConn) -> QueryResult<Vec<Self>> {
        asset::table.load::<Asset>(conn)
    }

    pub fn get_by_id(conn: &mut DatabaseConn, id: i32) -> QueryResult<Self> {
        asset::table.filter(dsl::id.eq(id)).first(conn)
    }

    pub fn get_by_title(conn: &mut DatabaseConn, title_id: i32) -> QueryResult<Vec<Self>> {
        asset::table.filter(dsl::title_id.eq(title_id)).load::<Asset>(conn)
    }

    pub fn create(conn: &mut DatabaseConn, asset: &AssetCreateDTO) -> QueryResult<usize> {
        insert_into(asset::dsl::asset).values(asset).execute(conn)
    }

    pub fn update(conn: &mut DatabaseConn, id: i32, asset: &AssetUpdateDTO) -> QueryResult<usize> {
        update(dsl::asset.find(id)).set(asset).execute(conn)
    }

    pub fn delete(conn: &mut DatabaseConn, id: i32) -> QueryResult<usize> {
        delete(dsl::asset.find(id)).execute(conn)
    }
}
