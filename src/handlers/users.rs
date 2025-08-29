use actix_web::{ web, Error, error, HttpResponse };
use serde_json::json;
use crate::{app_state::AppState, models::users};

pub fn config (cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users")
            .route(web::get().to(get_all))
    );
}

async fn get_all(pool: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let mut conn = pool.db.get().map_err(|_| error::ErrorInternalServerError("Failed to get DB connection"))?;

    let result = users::Users::get_all(&mut conn).map_err(|err| error::ErrorInternalServerError(err))?;

    Ok(HttpResponse::Ok().json(json!({ "status": "SUCCESS", "message": "", "content": result })))
}