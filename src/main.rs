#[macro_use]
extern crate diesel;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate log;

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};

mod api;
mod config;
mod constants;
mod error;
mod middleware;
mod model;
mod prelude;
mod schema;
mod service;
mod utils;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("Failed to read `.env` file");

    std::env::set_var("RUST_LOG", "actix_web=debug,paste_cross=debug");
    env_logger::init();

    let app_host = std::env::var("APP_HOST").expect("APP_HOST not found.");
    let app_port = std::env::var("APP_PORT").expect("APP_PORT not found.");
    let app_url = format!("{}:{}", &app_host, &app_port);

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_pool = config::db::new_pool(database_url).expect("Failed to create pool.");

    let app = move || {
        App::new()
            .data(database_pool.clone())
            .wrap(Logger::default())
            .wrap(middleware::authentication_middleware::Authentication)
            .configure(config::app::config_services)
    };

    HttpServer::new(app).bind(app_url)?.run().await
}
