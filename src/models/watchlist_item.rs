use diesel::{ prelude::Insertable, Queryable };
use serde::{ Serialize, Deserialize };
use uuid::Uuid;
use chrono::NaiveDateTime;

use crate::schema::watchlist_item;

#[derive(Queryable, Serialize, Deserialize)]
pub struct WatchlistItem {
    pub user_id: Uuid,
    pub title_id: Uuid,
    pub added_at: Option<NaiveDateTime>
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = watchlist_item)]
pub struct WatchlistItemDTO {
    pub user_id: Uuid,
    pub title_id: Uuid,
}
