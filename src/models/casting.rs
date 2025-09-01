use diesel::{ prelude::{Insertable, AsChangeset}, Queryable };
use serde::{ Serialize, Deserialize };

use crate::schema::casting;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Casting {
    pub title_id: i32,
    pub filmmaker_id: i32,
    pub role_id: i32,
    pub character_name: Option<String>
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = casting)]
pub struct CastingCreateDTO {
    pub title_id: i32,
    pub filmmaker_id: i32,
    pub role_id: i32,
    pub character_name: Option<String>
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = casting)]
pub struct CastingUpdateDTO {
    pub character_name: Option<String>
}
