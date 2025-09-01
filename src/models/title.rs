use diesel::{ prelude::{Insertable, AsChangeset}, Queryable };
use serde::{ Serialize, Deserialize };

use crate::schema::title;
use super::profile::MaturityRating;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Title {
    pub id: i32,
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
pub struct TitleCreateDTO {
    pub name: String,
    pub synopsis: String,
    pub release_year: i16,
    pub runtime_min: i16,
    pub age_rating: MaturityRating,
    pub poster_url: String,
    pub hero_image_url: String,
    pub orginal_language_id: Option<i32>,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = title)]
pub struct TitleUpdateDTO {
    pub name: Option<String>,
    pub synopsis: Option<String>,
    pub release_year: Option<i16>,
    pub runtime_min: Option<i16>,
    pub age_rating: Option<MaturityRating>,
    pub poster_url: Option<String>,
    pub hero_image_url: Option<String>,
    pub orginal_language_id: Option<i32>,
    pub is_active: Option<bool>
}
