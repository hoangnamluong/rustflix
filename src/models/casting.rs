use diesel::{ prelude::Insertable, Queryable };
use serde::{ Serialize, Deserialize };
use uuid::Uuid;

use crate::schema::casting;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Casting {
    pub title_id: Uuid,
    pub filmmaker_id: Uuid,
    pub role_id: Uuid,
    pub character_name: Option<String>
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = casting)]
pub struct CastingDTO {
    pub title_id: Uuid,
    pub filmmaker_id: Uuid,
    pub role_id: Uuid,
    pub character_name: Option<String>
}
