use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;

mod config;
use config::{
    log_config,
    db_config
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    log_config::init();

    let pool = db_config::get_connection_pool();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
} 