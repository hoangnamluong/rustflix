use diesel::{ prelude::{Insertable, AsChangeset}, Queryable };
use serde::{ Serialize, Deserialize };
use chrono::NaiveDateTime;

use crate::schema::playback_progress;

#[derive(Queryable, Serialize, Deserialize)]
pub struct PlaybackProgress {
    pub user_id: i32,
    pub asset_id: i32,
    pub position_ms: i32,
    pub updated_at: Option<NaiveDateTime>
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = playback_progress)]
pub struct PlaybackProgressCreateDTO {
    pub user_id: i32,
    pub asset_id: i32,
    pub position_ms: i32,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = playback_progress)]
pub struct PlaybackProgressUpdateDTO {
    pub position_ms: Option<i32>,
    pub updated_at: Option<NaiveDateTime>
}
