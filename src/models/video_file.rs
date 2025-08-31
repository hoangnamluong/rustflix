use diesel::{ prelude::Insertable, Queryable };
use serde::{ Serialize, Deserialize };
use uuid::Uuid;

use crate::schema::video_file;

#[derive(Queryable, Serialize, Deserialize)]
pub struct VideoFile {
    pub id: Uuid,
    pub asset_id: Option<Uuid>,
    pub codec: String,
    pub container: String,
    pub width: i32,
    pub height: i32,
    pub bitrate_kbps: i32
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = video_file)]
pub struct VideoFileDTO {
    pub asset_id: Option<Uuid>,
    pub codec: String,
    pub container: String,
    pub width: i32,
    pub height: i32,
    pub bitrate_kbps: i32
}
