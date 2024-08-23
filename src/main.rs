mod controllers;
mod models;
mod entities;
use std::time::Duration;

use actix_web::{App, HttpServer};
use log;
use sea_orm::{ConnectOptions, Database};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut opt = ConnectOptions::new("sqlite://example.db?mode=rwc");
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
    HttpServer::new(|| App::new().route("/"))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
