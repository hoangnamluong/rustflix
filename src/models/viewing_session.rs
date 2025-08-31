use diesel::{ prelude::Insertable, Queryable };
use chrono::NaiveDateTime;
use serde::{ Serialize, Deserialize };
use uuid::Uuid;

use crate::schema::viewing_session;

#[derive(Queryable, Serialize, Deserialize)]
pub struct ViewingSession {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub asset_id: Option<Uuid>,
    pub started_at: Option<NaiveDateTime>,
    pub ended_at: Option<NaiveDateTime>
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = viewing_session)]
pub struct ViewingSessionDTO {
    pub user_id: Option<Uuid>,
    pub asset_id: Option<Uuid>,
    pub started_at: Option<NaiveDateTime>,
    pub ended_at: Option<NaiveDateTime>
}
