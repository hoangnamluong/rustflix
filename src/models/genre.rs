use diesel::{ prelude::Insertable, Queryable };
use serde::{ Serialize, Deserialize };
use uuid::Uuid;

use crate::schema::genre;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Genre {
    pub id: Uuid,
    pub name: String
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = genre)]
pub struct GenreDTO {
    pub name: String
}
