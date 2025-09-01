use diesel::{ prelude::{Insertable, AsChangeset}, Queryable };
use diesel_derive_enum::DbEnum;
use serde::{ Serialize, Deserialize };

use crate::schema::asset;

#[derive(DbEnum, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[ExistingTypePath = "crate::schema::sql_types::StreamingProtocol"]
pub enum StreamingProtocol {
    #[db_rename = "DASH"]
    DASH,
    #[db_rename = "HLS"]
    HLS
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Asset {
    pub id: i32,
    pub title_id: Option<i32>,
    pub manifest_url: StreamingProtocol,
    pub subtitle_locales: Option<i32>,
    pub audio_locales: Option<i32>
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = asset)]
pub struct AssetCreateDTO {
    pub title_id: Option<i32>,
    pub manifest_url: StreamingProtocol,
    pub subtitle_locales: Option<i32>,
    pub audio_locales: Option<i32>
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = asset)]
pub struct AssetUpdateDTO {
    pub title_id: Option<i32>,
    pub manifest_url: Option<StreamingProtocol>,
    pub subtitle_locales: Option<i32>,
    pub audio_locales: Option<i32>
}
