mod commons;
mod controllers;
mod requests;
mod services;
use std::time::Duration;

use actix_web::{middleware::Logger, App, HttpServer};
use log;
use sea_orm::{ConnectOptions, Database};
use services::AuthService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let mut opt =
        ConnectOptions::new("postgres://postgres:postgres@localhost:5432/http_server_rust");
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
        App::new().wrap(Logger::default())
            .app_data(actix_web::web::Data::new(AuthService::new(db.clone())))
            .configure(controllers::auth_controller::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
