mod app;
mod config;
mod handlers;
mod services;

use crate::config::api::{Config, CONFIG};
use actix_web::{middleware::Logger, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = Config::establish_connection(&CONFIG);
    let _ = Config::log();

    log::info!("Running: {}:{}", CONFIG.api.host, CONFIG.api.port);

    HttpServer::new(move || App::new().wrap(Logger::default()).configure(app::app))
        .bind(format!("{}:{}", CONFIG.api.host, CONFIG.api.port))?
        .run()
        .await
}
