use diesel::{ prelude::{Insertable, AsChangeset}, Queryable };
use serde::{ Serialize, Deserialize };

use crate::schema::genre;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Genre {
    pub id: i32,
    pub name: String
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = genre)]
pub struct GenreCreateDTO {
    pub name: String
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = genre)]
pub struct GenreUpdateDTO {
    pub name: Option<String>
}
