use diesel::{ prelude::Insertable, Queryable };
use serde::{ Serialize, Deserialize };
use uuid::Uuid;

use crate::schema::filmmaker;
use chrono::NaiveDate;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Filmmaker {
    pub id: Uuid,
    pub name: String,
    pub bio: Option<String>,
    pub birth_date: Option<NaiveDate>
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = filmmaker)]
pub struct FilmmakerDTO {
    pub name: String,
    pub bio: Option<String>,
    pub birth_date: Option<NaiveDate>
}
