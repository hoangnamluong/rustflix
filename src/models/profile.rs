use diesel::{ prelude::{Insertable, AsChangeset}, Queryable };
use diesel_derive_enum::DbEnum;
use serde::{ Serialize, Deserialize };

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
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub maturity_rating_max: MaturityRating,
    pub avatar_url: Option<String>,
    pub language: i32
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = profile)]
pub struct ProfileCreateDTO {
    pub user_id: i32,
    pub name: String,
    pub maturity_rating_max: MaturityRating,
    pub avatar_url: Option<String>,
    pub language: i32
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = profile)]
pub struct ProfileUpdateDTO {
    pub name: Option<String>,
    pub maturity_rating_max: Option<MaturityRating>,
    pub avatar_url: Option<String>,
    pub language: Option<i32>
}