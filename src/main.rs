use actix_web::{get, web, App, HttpServer, Responder};
use actix_web::middleware::Logger;
use serde_derive::Deserialize;

#[macro_use]
extern crate log;

#[derive(Debug, Deserialize)]
struct Params {
    test: String,
    name: String,
}

#[get("/")]
async fn index(info: web::Query<Params>) -> impl Responder {
    info!("Hello {:?}", info);
    return format!("Hello {:?}", info);
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug,paste_cross=debug");
    env_logger::init();

    let ip_address = "127.0.0.1:8080";
    println!("Running server on {}", ip_address);

    let app = move || {
        debug!("Constructing the App");
        App::new()
            .wrap(Logger::default())
            .service(index)
    };


    return HttpServer::new(app)
        .bind(ip_address)?
        .run()
        .await;
}