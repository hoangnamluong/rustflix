use diesel::{ prelude::{Insertable, AsChangeset}, Queryable };
use serde::{ Serialize, Deserialize };
use chrono::NaiveDateTime;

use crate::schema::watchlist_item;

#[derive(Queryable, Serialize, Deserialize)]
pub struct WatchlistItem {
    pub user_id: i32,
    pub title_id: i32,
    pub added_at: Option<NaiveDateTime>
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = watchlist_item)]
pub struct WatchlistItemCreateDTO {
    pub user_id: i32,
    pub title_id: i32,
    pub added_at: Option<NaiveDateTime>
}

// #[derive(AsChangeset, Deserialize)]
// #[diesel(table_name = watchlist_item)]
// pub struct WatchlistItemUpdateDTO {
//     pub added_at: Option<NaiveDateTime>
// }
