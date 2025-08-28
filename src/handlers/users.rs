use actix_web::{get, post, web, Error, HttpResponse, Responder};
use crate::config::db_config::DatabasePool;
use serde_json::json;

#[get("/users")]
pub async fn get_all(pool: web::Data<DatabasePool>) -> impl Responder {
    // let mut conn = pool.get().map_err(|_| actix_web::error::ErrorInternalServerError("Failed to get DB connection"));    

    HttpResponse::Ok().json(json!({ "status": "SUCCESS", "message": "" }))
}