use actix_web::{ error::{self, ErrorInternalServerError}, web, Error, HttpResponse };
use crate::{app_state::AppState, models::users::{self, UsersDTO}, utils::api_response::ApiResponse};

pub fn config (cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users")
            .route(web::get().to(get_all))
            .route(web::post().to(create))
    );
}

async fn get_all(pool: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let mut conn = pool.db.get().map_err(|_| error::ErrorInternalServerError("Failed to get DB connection"))?;

    match users::Users::get_all(&mut conn) {
        Ok(content) => Ok(HttpResponse::Ok().json(ApiResponse::success(content))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<&'_ str>::error(&err.to_string())))
    }
}

async fn create(pool: web::Data<AppState>, user: web::Json<UsersDTO>) -> Result<HttpResponse, Error> {
    let mut conn = pool.db.get().map_err(|_| error::ErrorInternalServerError("Failed to get DB connection"))?;

    match users::Users::create(&mut conn, &user) {
        Ok(content) => Ok(HttpResponse::Ok().json(ApiResponse::success(content))),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<&'_ str>::error(&err.to_string())))
    }
}