use diesel::{ prelude::Insertable, Queryable };
use diesel_derive_enum::DbEnum;
use serde::{ Serialize, Deserialize };
use uuid::Uuid;

use crate::schema::profile;

#[derive(DbEnum, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[ExistingTypePath = "crate::schema::sql_types::MaturityRating"]
pub enum MaturityRating {
    #[db_rename = "G"]
    G,
    #[db_rename = "PG"]
    PG,
    #[db_rename = "PG13"]
    PG13,
    #[db_rename = "R"]
    R,
    #[db_rename = "NC17"]
    NC17
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Profile {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub maturity_rating_max: MaturityRating,
    pub avatar_url: Option<String>,
    pub language: i32
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = profile)]
pub struct ProfileDTO {
    pub user_id: Uuid,
    pub name: String,
    pub maturity_rating_max: MaturityRating,
    pub avatar_url: Option<String>,
    pub language: i32
}