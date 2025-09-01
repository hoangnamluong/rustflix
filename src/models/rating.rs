use diesel::{ prelude::{Insertable, AsChangeset}, Queryable };
use serde::{ Serialize, Deserialize };
use chrono::NaiveDateTime;

use crate::schema::rating;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Rating {
    pub id: i32,
    pub user_id: Option<i32>,
    pub title_id: Option<i32>,
    pub score: i16,
    pub rated_at: Option<NaiveDateTime>
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = rating)]
pub struct RatingCreateDTO {
    pub user_id: Option<i32>,
    pub title_id: Option<i32>,
    pub score: i16,
    pub rated_at: Option<NaiveDateTime>
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = rating)]
pub struct RatingUpdateDTO {
    pub score: Option<i16>,
    pub rated_at: Option<NaiveDateTime>
}
