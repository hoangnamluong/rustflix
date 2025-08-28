use chrono::NaiveDateTime;
use diesel::{ Queryable };
use diesel_derive_enum::DbEnum;
use serde::{ Serialize, Deserialize };
use uuid::Uuid;

#[derive(DbEnum, Debug, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::UserStatus"]
pub enum UserStatus {
    ACTIVE, 
    BANNED, 
    INACTIVE
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Users {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub status: UserStatus,
    pub created_at: NaiveDateTime,
    pub country_id: Option<i32>
}