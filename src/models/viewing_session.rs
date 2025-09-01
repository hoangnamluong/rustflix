use diesel::{ prelude::{Insertable, AsChangeset}, Queryable };
use chrono::NaiveDateTime;
use serde::{ Serialize, Deserialize };

use crate::schema::viewing_session;

#[derive(Queryable, Serialize, Deserialize)]
pub struct ViewingSession {
    pub id: i32,
    pub user_id: Option<i32>,
    pub asset_id: Option<i32>,
    pub started_at: Option<NaiveDateTime>,
    pub ended_at: Option<NaiveDateTime>
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = viewing_session)]
pub struct ViewingSessionCreateDTO {
    pub user_id: Option<i32>,
    pub asset_id: Option<i32>,
    pub started_at: Option<NaiveDateTime>,
    pub ended_at: Option<NaiveDateTime>
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = viewing_session)]
pub struct ViewingSessionUpdateDTO {
    pub started_at: Option<NaiveDateTime>,
    pub ended_at: Option<NaiveDateTime>
}
