use chrono::NaiveDateTime;
use diesel::{ Queryable };
use diesel_derive_enum::DbEnum;
use serde::{ Serialize, Deserialize };
use uuid::Uuid;

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
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub status: UserStatus,
    pub created_at: NaiveDateTime,
    pub country_id: Option<i32>
}