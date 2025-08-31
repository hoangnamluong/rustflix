use diesel::{ prelude::Insertable, Queryable };
use serde::{ Serialize, Deserialize };
use uuid::Uuid;
use chrono::NaiveDateTime;

use crate::schema::rating;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Rating {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub title_id: Option<Uuid>,
    pub score: i16,
    pub rated_at: Option<NaiveDateTime>
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = rating)]
pub struct RatingDTO {
    pub user_id: Option<Uuid>,
    pub title_id: Option<Uuid>,
    pub score: i16,
}
