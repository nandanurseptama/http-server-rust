mod commons;
mod controllers;
mod requests;
mod services;
mod responses;
use std::time::Duration;

use actix_web::{middleware::Logger, App, HttpServer};
use log;
use sea_orm::{ConnectOptions, Database};
use services::AuthService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();
    let config = commons::config::Config::new();

    let jwt = commons::jwt::JWT::new(config.auth_token_secret, config.refresh_token_secret);

    let mut opt =
        ConnectOptions::new(config.db_url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);
    let db = Database::connect(opt).await.unwrap();

    log::info!("db connected");
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(actix_web::web::Data::new(AuthService::new(
                db.clone(),
                jwt.clone(),
            )))
            .configure(controllers::auth_controller::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
