use chrono::NaiveDateTime;
use diesel::{ prelude::{Insertable, AsChangeset}, Queryable };
use diesel_derive_enum::DbEnum;
use serde::{ Serialize, Deserialize };
use uuid::Uuid;

use crate::schema::users;

#[derive(DbEnum, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[ExistingTypePath = "crate::schema::sql_types::UserStatus"]
pub enum UserStatus {
    #[db_rename = "ACTIVE"]
    ACTIVE,
    #[db_rename = "BANNED"]
    BANNED,
    #[db_rename = "INACTIVE"]
    INACTIVE
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Users {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub status: UserStatus,
    pub created_at: NaiveDateTime,
    pub country_id: Option<i32>
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct UsersCreateDTO {
    pub email: String,
    pub password: String,
    pub country_id: Option<i32>
}

#[derive(AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct UsersUpdateDTO {
    pub email: Option<String>,
    pub password: Option<String>
}