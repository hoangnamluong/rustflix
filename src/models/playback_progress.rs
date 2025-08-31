use diesel::{ prelude::Insertable, Queryable };
use serde::{ Serialize, Deserialize };
use uuid::Uuid;
use chrono::NaiveDateTime;

use crate::schema::playback_progress;

#[derive(Queryable, Serialize, Deserialize)]
pub struct PlaybackProgress {
    pub user_id: Uuid,
    pub asset_id: Uuid,
    pub position_ms: i32,
    pub updated_at: Option<NaiveDateTime>
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = playback_progress)]
pub struct PlaybackProgressDTO {
    pub user_id: Uuid,
    pub asset_id: Uuid,
    pub position_ms: i32,
}
