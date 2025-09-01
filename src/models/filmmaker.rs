use diesel::{ prelude::{Insertable, AsChangeset}, Queryable };
use serde::{ Serialize, Deserialize };

use crate::schema::filmmaker;
use chrono::NaiveDate;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Filmmaker {
    pub id: i32,
    pub name: String,
    pub bio: Option<String>,
    pub birth_date: Option<NaiveDate>
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = filmmaker)]
pub struct FilmmakerCreateDTO {
    pub name: String,
    pub bio: Option<String>,
    pub birth_date: Option<NaiveDate>
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = filmmaker)]
pub struct FilmmakerUpdateDTO {
    pub name: Option<String>,
    pub bio: Option<String>,
    pub birth_date: Option<NaiveDate>
}
