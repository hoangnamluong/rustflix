use diesel::{ prelude::Insertable, Queryable };
use serde::{ Serialize, Deserialize };
use uuid::Uuid;

use crate::schema::casting_role;

#[derive(Queryable, Serialize, Deserialize)]
pub struct CastingRole {
    pub id: Uuid,
    pub name: String
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = casting_role)]
pub struct CastingRoleDTO {
    pub name: String
}
