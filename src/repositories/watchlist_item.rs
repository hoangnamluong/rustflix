use diesel::dsl::{delete, insert_into, update};
use diesel::query_dsl::methods::{FilterDsl, FindDsl};
use diesel::{ExpressionMethods, QueryResult, RunQueryDsl};
use crate::config::db_config::DatabaseConn;
use crate::models::watchlist_item::{WatchlistItem, WatchlistItemCreateDTO};
use crate::schema::watchlist_item::{self, dsl};

impl WatchlistItem {
    pub fn get_all(conn: &mut DatabaseConn) -> QueryResult<Vec<Self>> {
        watchlist_item::table.load::<WatchlistItem>(conn)
    }

    pub fn get_by_user(conn: &mut DatabaseConn, user_id: i32) -> QueryResult<Vec<Self>> {
        watchlist_item::table.filter(dsl::user_id.eq(user_id)).load::<WatchlistItem>(conn)
    }

    pub fn get_by_title(conn: &mut DatabaseConn, title_id: i32) -> QueryResult<Vec<Self>> {
        watchlist_item::table.filter(dsl::title_id.eq(title_id)).load::<WatchlistItem>(conn)
    }

    pub fn create(conn: &mut DatabaseConn, item: &WatchlistItemCreateDTO) -> QueryResult<usize> {
        insert_into(watchlist_item::dsl::watchlist_item).values(item).execute(conn)
    }

    /*
        No need to update
        => whenever user presses/clicks add to watchlist or remove from watchlist button 
        => create new watchlist row or delete existing watchlist row
     */
    // pub fn update(conn: &mut DatabaseConn, user_id: i32, title_id: i32, item: &WatchlistItemUpdateDTO) -> QueryResult<usize> {
    //     update(watchlist_item::table.filter(dsl::user_id.eq(user_id))
    //         .filter(dsl::title_id.eq(title_id)))
    //         .set(item)
    //         .execute(conn)
    // }

    pub fn delete(conn: &mut DatabaseConn, user_id: i32, title_id: i32) -> QueryResult<usize> {
        delete(watchlist_item::table.filter(dsl::user_id.eq(user_id))
            .filter(dsl::title_id.eq(title_id)))
            .execute(conn)
    }
}
