use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginDTO {
    pub email: String,
    pub password: String,
}

pub struct RegisterDTO {
    pub email: String,
    pub password: String,
    pub locale: String,
}