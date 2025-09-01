use diesel::{ prelude::{Insertable, AsChangeset}, Queryable };
use serde::{ Serialize, Deserialize };

use crate::schema::video_file;

#[derive(Queryable, Serialize, Deserialize)]
pub struct VideoFile {
    pub id: i32,
    pub asset_id: Option<i32>,
    pub codec: String,
    pub container: String,
    pub width: i32,
    pub height: i32,
    pub bitrate_kbps: i32
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = video_file)]
pub struct VideoFileCreateDTO {
    pub asset_id: Option<i32>,
    pub codec: String,
    pub container: String,
    pub width: i32,
    pub height: i32,
    pub bitrate_kbps: i32
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = video_file)]
pub struct VideoFileUpdateDTO {
    pub codec: Option<String>,
    pub container: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub bitrate_kbps: Option<i32>
}
