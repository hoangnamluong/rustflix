use diesel::{ prelude::{Insertable, AsChangeset}, Queryable };
use serde::{ Serialize, Deserialize };

use crate::schema::casting_role;

#[derive(Queryable, Serialize, Deserialize)]
pub struct CastingRole {
    pub id: i32,
    pub name: String
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = casting_role)]
pub struct CastingRoleCreateDTO {
    pub name: String
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = casting_role)]
pub struct CastingRoleUpdateDTO {
    pub name: Option<String>
}
