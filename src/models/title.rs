use diesel::{ prelude::Insertable, Queryable };
use serde::{ Serialize, Deserialize };
use uuid::Uuid;

use crate::schema::title;
use super::profile::MaturityRating;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Title {
    pub id: Uuid,
    pub name: String,
    pub synopsis: String,
    pub release_year: i16,
    pub runtime_min: i16,
    pub age_rating: MaturityRating,
    pub poster_url: String,
    pub hero_image_url: String,
    pub orginal_language_id: Option<i32>,
    pub is_active: bool
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = title)]
pub struct TitleDTO {
    pub name: String,
    pub synopsis: String,
    pub release_year: i16,
    pub runtime_min: i16,
    pub age_rating: MaturityRating,
    pub poster_url: String,
    pub hero_image_url: String,
    pub orginal_language_id: Option<i32>,
}
