use crate::api::*;
use actix_web::web;

pub fn config_services(cfg: &mut web::ServiceConfig) {
    info!("");
    cfg.service(
        web::scope("/api/")
            //            .service(ping_controller::ping)
            .service(web::resource("/auth/login").route(web::post().to(account::login))),
    );
}
