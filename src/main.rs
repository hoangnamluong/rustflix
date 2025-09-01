use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenvy::dotenv;
use rustflix::config::{
    db_config, 
    log_config
};
use rustflix::app_state::AppState;
use rustflix::handlers::{
    asset,
    casting_role,
    casting,
    filmmaker,
    genre,
    playback_progress,
    profile,
    rating,
    title,
    users, 
    video_file,
    viewing_session,
    watchlist_item
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    log_config::init();

    let pool: db_config::DatabasePool = db_config::get_connection_pool();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .supports_credentials();

        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .service(
                web::scope("/api/v1")
                    .configure(users::config)
                    .configure(profile::config)
                    .configure(title::config)
                    .configure(genre::config)
                    .configure(filmmaker::config)
                    .configure(casting::config)
                    .configure(asset::config)
                    .configure(video_file::config)
                    .configure(viewing_session::config)
                    .configure(watchlist_item::config)
                    .configure(rating::config)
                    .configure(casting_role::config)
                    .configure(playback_progress::config)
            )
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
} 